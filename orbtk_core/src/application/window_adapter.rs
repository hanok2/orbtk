use std::{cell::RefCell, sync::mpsc};

use dces::prelude::*;

use crate::{
    application::*,
    event::*,
    localization::Localization,
    render,
    services::{Clipboard, Settings},
    shell,
    shell::{ShellRequest, WindowRequest, WindowSettings},
    systems::*,
    theming::Theme,
    tree::Tree,
    utils::{Constraint, Point, Rectangle},
    widget_base::*,
};

/// Represents a window.
///
/// Each window has associated its unique tree of enities, an event pipeline and a shell.
///

pub struct WindowAdapter {
    world: World<Tree, render::RenderContext2D>,
    ctx: ContextProvider,
    registry: Rc<RefCell<Registry>>,
    old_clipboard_value: Option<String>,
}

impl WindowAdapter {
    /// Creates a new WindowAdapter.
    pub fn new(
        world: World<Tree, render::RenderContext2D>,
        ctx: ContextProvider,
        registry: Rc<RefCell<Registry>>,
    ) -> Self {
        WindowAdapter {
            world,
            ctx,
            registry,
            old_clipboard_value: None,
        }
    }
}

impl WindowAdapter {
    fn root(&mut self) -> Entity {
        self.world
            .entity_component_manager()
            .entity_store()
            .root
            .unwrap()
    }
}

impl shell::WindowAdapter for WindowAdapter {
    fn active(&mut self, active: bool) {
        let root = self.root();

        self.ctx
            .event_adapter
            .push_event_direct(root, WindowEvent::ActiveChanged(active));
    }

    fn clipboard_update(&mut self, value: &mut Option<String>) {
        // internal clipboard value is new => update system clipboard value.
        if self.registry.borrow().get::<Clipboard>("clipboard").get() != self.old_clipboard_value {
            *value = self.registry.borrow().get::<Clipboard>("clipboard").get();

            self.old_clipboard_value = value.clone();

            return;
        }

        //  system clipboard value is newer => update internal clipboard
        if let Some(value) = value.clone() {
            self.registry
                .borrow_mut()
                .get_mut::<Clipboard>("clipboard")
                .set(value.clone());
            self.old_clipboard_value = Some(value);
        }
    }

    fn file_drop_event(&mut self, file_name: String) {
        let root = self.root();
        self.ctx.event_adapter.push_event(
            root,
            DropFileEvent {
                file_name,
                position: self.mouse_position(),
            },
        );
    }

    fn key_event(&mut self, event: shell::KeyEvent) {
        let root = self.root();
        match event.state {
            shell::ButtonState::Up => self
                .ctx
                .event_adapter
                .push_event(root, KeyUpEvent { event }),
            shell::ButtonState::Down => {
                self.ctx
                    .event_adapter
                    .push_event(root, KeyDownEvent { event });
            }
        }
    }

    fn mouse(&mut self, x: f64, y: f64) {
        let root = self.root();
        self.ctx.mouse_position.set(Point::new(x, y));
        self.ctx.event_adapter.push_event(
            root,
            MouseMoveEvent {
                position: Point::new(x, y),
            },
        );
    }

    fn mouse_event(&mut self, event: shell::MouseEvent) {
        let root = self.root();
        match event.state {
            shell::ButtonState::Up => {
                self.ctx.event_adapter.push_event(
                    root,
                    MouseUpEvent {
                        position: event.position,
                        button: event.button,
                    },
                );
                self.ctx.event_adapter.push_event(
                    root,
                    GlobalMouseUpEvent {
                        position: event.position,
                        button: event.button,
                    },
                );
            }
            shell::ButtonState::Down => self.ctx.event_adapter.push_event(
                root,
                MouseDownEvent {
                    position: event.position,
                    button: event.button,
                },
            ),
        }
    }

    fn mouse_position(&self) -> Point {
        self.ctx.mouse_position.get()
    }

    fn quit_event(&mut self) {
        let root = self.root();

        self.ctx
            .event_adapter
            .push_event_direct(root, SystemEvent::Quit);
    }

    fn resize(&mut self, width: f64, height: f64) {
        let root = self.root();
        // TODO: respect min.width and min.height when resizing the window.
        //       right now, we are able to downscale to bounds of (0.0, 0.0)

        self.ctx
            .event_adapter
            .push_event_direct(root, WindowEvent::Resize { width, height });
    }

    fn run(&mut self, _render_context: &mut render::RenderContext2D) {
        self.world.run();
    }

    fn scroll(&mut self, delta_x: f64, delta_y: f64) {
        let root = self.root();
        self.ctx.event_adapter.push_event(
            root,
            ScrollEvent {
                delta: Point::new(delta_x, delta_y),
            },
        );
    }

    fn set_raw_window_handle(&mut self, raw_window_handle: raw_window_handle::RawWindowHandle) {
        self.ctx.raw_window_handle = Some(raw_window_handle);
    }

    fn text_input(&mut self, text: String) {
        let root = self.root();
        self.ctx
            .event_adapter
            .push_event(root, TextInputEvent { text });
    }

    fn text_drop_event(&mut self, text: String) {
        let root = self.root();
        self.ctx.event_adapter.push_event(
            root,
            DropTextEvent {
                text,
                position: self.ctx.mouse_position.get(),
            },
        );
    }
}

/// Uses the window builder closure to creates a new `WindowAdapter`
/// and a new `WindowSettings` object.
///
/// WindowsAdapter: Handler that manages the widet tree of the windows.
/// WindowsSettings: presets the WindowAdapter with stored application settings.
/// ContextProvider will respect any localization settings.
pub fn create_window<F: Fn(&mut BuildContext) -> Entity + 'static>(
    app_name: impl Into<String>,
    theme: &Rc<Theme>,
    request_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    create_fn: F,
    localization: Option<Rc<RefCell<Box<dyn Localization>>>>,
) -> (WindowAdapter, WindowSettings, mpsc::Receiver<WindowRequest>) {
    #[cfg(not(feature = "debug"))]
    let _debug = false;

    let app_name = app_name.into();
    let mut world: World<Tree, render::RenderContext2D> = World::from_entity_store(Tree::default());

    let (sender, receiver) = mpsc::channel();

    let registry = Rc::new(RefCell::new(Registry::new()));

    let context_provider =
        ContextProvider::new(sender, request_sender, app_name.clone(), localization);

    // Register the window settings via the context_provider.
    if app_name.is_empty() {
        registry.borrow_mut().register(
            "settings",
            Settings::new(context_provider.message_adapter.clone()),
        );
    } else {
        registry.borrow_mut().register(
            "settings",
            Settings::from_name(app_name, context_provider.message_adapter.clone()),
        );
    };

    // Register a dedicated window assigned clipboard.
    registry
        .borrow_mut()
        .register("clipboard", Clipboard::new());

    // Assing an Overlay, that draws the root window on top of all
    // other childs of the window widget tree.
    let window = {
        let overlay = Overlay::new().build(&mut BuildContext::new(
            world.entity_component_manager(),
            &context_provider.render_objects,
            &context_provider.layouts,
            &context_provider.handler_map,
            &mut *context_provider.states.borrow_mut(),
            theme,
            context_provider.event_adapter.clone(),
        ));

        {
            let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
            tree.set_overlay(overlay);
        }

        // Use the BuilderContext to create a new window as the root entity.
        let window = create_fn(&mut BuildContext::new(
            world.entity_component_manager(),
            &context_provider.render_objects,
            &context_provider.layouts,
            &context_provider.handler_map,
            &mut *context_provider.states.borrow_mut(),
            theme,
            context_provider.event_adapter.clone(),
        ));

        {
            let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
            tree.set_root(window);
        }

        // Return the window entity
        window
    };

    let constraint = *world
        .entity_component_manager()
        .component_store()
        .get::<Constraint>("constraint", window)
        .unwrap();

    let position = *world
        .entity_component_manager()
        .component_store()
        .get::<Point>("position", window)
        .unwrap();

    // Consume theme specific fonts.
    let fonts = theme.fonts().clone();

    // Consume stored application settings for the corresponding components.
    let settings = WindowSettings {
        title: world
            .entity_component_manager()
            .component_store()
            .get::<String>("title", window)
            .unwrap()
            .clone(),
        borderless: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("borderless", window)
            .unwrap(),
        resizeable: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("resizable", window)
            .unwrap(),
        always_on_top: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("always_on_top", window)
            .unwrap(),
        position: (position.x(), position.y()),
        size: (constraint.width(), constraint.height()),
        fonts,
    };

    // let mut global = Global::default();
    // global.theme = theme;

    // Construct the `component` store registered via the ECM.
    world
        .entity_component_manager()
        .component_store_mut()
        .register("theme", window, Rc::clone(theme));
    world
        .entity_component_manager()
        .component_store_mut()
        .register(
            "bounds",
            window,
            Rectangle::from((0.0, 0.0, constraint.width(), constraint.height())),
        );

    // Register and activate the application `systems`.
    world.register_init_system(InitSystem::new(context_provider.clone(), registry.clone()));

    world.register_cleanup_system(CleanupSystem::new(
        context_provider.clone(),
        registry.clone(),
    ));

    world
        .create_system(EventStateSystem::new(
            context_provider.clone(),
            registry.clone(),
            RefCell::new(vec![]),
        ))
        .with_priority(0)
        .build();

    world
        .create_system(LayoutSystem::new(context_provider.clone()))
        .with_priority(1)
        .build();

    world
        .create_system(PostLayoutStateSystem::new(
            context_provider.clone(),
            registry.clone(),
        ))
        .with_priority(2)
        .build();

    world
        .create_system(RenderSystem::new(context_provider.clone()))
        .with_priority(3)
        .build();

    (
        WindowAdapter::new(world, context_provider, registry),
        settings,
        receiver,
    )
}
