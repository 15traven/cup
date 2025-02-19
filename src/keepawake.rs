use windows::{
    core::Error as WindowsError,
    Win32::System::Power::{
        SetThreadExecutionState, 
        ES_DISPLAY_REQUIRED, 
        ES_SYSTEM_REQUIRED,
        EXECUTION_STATE,
        ES_CONTINUOUS
    }
};

pub struct Options {
    pub display: bool,
    pub idle: bool
}

pub struct KeepAwake {
    options: Options,
    previous: EXECUTION_STATE
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        unsafe {
            SetThreadExecutionState(self.previous);
        }
    }
}

impl KeepAwake {
    pub fn new(options: Options) -> Result<Self, WindowsError> {
        let mut keepawake = KeepAwake {
            options,
            previous: Default::default()
        };

        keepawake.set()?;
        Ok(keepawake)
    }

    fn set(&mut self) -> Result<(), WindowsError> {
        let mut esflags = ES_CONTINUOUS;

        if self.options.display {
            esflags |= ES_DISPLAY_REQUIRED;
        }

        if self.options.idle {
            esflags |= ES_SYSTEM_REQUIRED;
        }

        unsafe {
            self.previous = SetThreadExecutionState(esflags);
            if self.previous == EXECUTION_STATE(0) {
                return Err(WindowsError::from_win32());
            }

            Ok(())
        }
    }
}