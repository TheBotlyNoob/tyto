// TODO: move to a USB controller
use ps2::{error::ControllerError, flags::ControllerConfigFlags, Controller};
use spin::{Lazy, Mutex};

pub static CONTROLLER: Lazy<Mutex<Controller>> = Lazy::new(|| {
    (|| -> Result<Mutex<Controller>, ControllerError> {
        let mut controller = unsafe { Controller::new() };

        // Step 3: Disable devices
        controller.disable_keyboard()?;
        controller.disable_mouse()?;

        // Step 4: Flush data buffer
        let _ = controller.read_data();

        // Step 5: Set config
        let mut config = controller.read_config()?;
        // Disable interrupts and scancode translation
        config.set(
            ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT
                | ControllerConfigFlags::ENABLE_MOUSE_INTERRUPT
                | ControllerConfigFlags::ENABLE_TRANSLATE,
            false,
        );
        controller.write_config(config)?;

        // Step 6: Controller self-test
        controller.test_controller()?;
        // Write config again in case of controller reset
        controller.write_config(config)?;

        // Step 7: Determine if there are 2 devices
        let has_mouse = if config.contains(ControllerConfigFlags::DISABLE_MOUSE) {
            controller.enable_mouse()?;
            config = controller.read_config()?;
            // If mouse is working, this should now be unset
            !config.contains(ControllerConfigFlags::DISABLE_MOUSE)
        } else {
            false
        };
        // Disable mouse. If there's no mouse, this is ignored
        controller.disable_mouse()?;

        // Step 8: Interface tests
        let keyboard_works = controller.test_keyboard().is_ok();
        let mouse_works = has_mouse && controller.test_mouse().is_ok();

        // Step 9 - 10: Enable and reset devices
        config = controller.read_config()?;
        if keyboard_works {
            controller.enable_keyboard()?;
            config.set(ControllerConfigFlags::DISABLE_KEYBOARD, false);
            config.set(ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT, true);
            controller.keyboard().reset_and_self_test().unwrap();
        }
        if mouse_works {
            controller.enable_mouse()?;
            config.set(ControllerConfigFlags::DISABLE_MOUSE, false);
            config.set(ControllerConfigFlags::ENABLE_MOUSE_INTERRUPT, true);
            controller.mouse().reset_and_self_test().unwrap();
            // This will start streaming events from the mouse
            controller.mouse().enable_data_reporting().unwrap();
        }
        Ok(Mutex::new(controller))
    })()
    .unwrap()
});
