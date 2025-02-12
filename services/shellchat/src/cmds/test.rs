use crate::{ShellCmdApi,CommonEnv};
use xous_ipc::String;

#[derive(Debug)]
pub struct Test {
    state: u32
}
impl Test {
    pub fn new() -> Self {
        Test {
            state: 0
        }
    }
}

impl<'a> ShellCmdApi<'a> for Test {
    cmd_api!(test);

    fn process(&mut self, args: String::<1024>, env: &mut CommonEnv) -> Result<Option<String::<1024>>, xous::Error> {
        use core::fmt::Write;

        const SENTINEL: &'static str = "|TSTR";

        self.state += 1;
        let mut ret = String::<1024>::new();
        write!(ret, "Test has run {} times.\n", self.state).unwrap();

        let mut tokens = args.as_str().unwrap().split(' ');

        if let Some(sub_cmd) = tokens.next() {
            match sub_cmd {
                "factory" => {
                    // set uart MUX, and turn off WFI so UART reports are "clean" (no stuck characters when CPU is in WFI)
                    env.llio.set_uart_mux(llio::UartType::Log).unwrap();
                    env.llio.wfi_override(true).unwrap();
                    let (x, y, z, id) = env.com.gyro_read_blocking().unwrap();
                    log::info!("{}|GYRO|{}|{}|{}|{}", SENTINEL, x, y, z, id);
                    let (wf_maj, wf_min, wf_rev) = env.com.get_wf200_fw_rev().unwrap();
                    log::info!("{}|WF200REV|{}|{}|{}", SENTINEL, wf_maj, wf_min, wf_rev);
                    let (ec_rev, ec_dirty) =  env.com.get_ec_git_rev().unwrap();
                    log::info!("{}|ECREV|{:x}|{:?}", SENTINEL, ec_rev, ec_dirty);
                    let morestats = env.com.get_more_stats().unwrap();
                    log::info!("{}|BATTSTATS|{:?}", SENTINEL, morestats);
                    let (usbcc_event, usbcc_regs) = env.com.poll_usb_cc().unwrap();
                    log::info!("{}|USBCC|{:?}|{:?}", SENTINEL, usbcc_event, usbcc_regs);

                    write!(ret, "Factory test script has run, check serial terminal for output").unwrap();
                    env.llio.wfi_override(false).unwrap();
                }
                "devboot" => {
                    env.gam.set_devboot(true).unwrap();
                    write!(ret, "devboot on").unwrap();
                }
                "devbootoff" => {
                    // this should do nothing if devboot was already set
                    env.gam.set_devboot(false).unwrap();
                    write!(ret, "devboot off").unwrap();
                }
                _ => {
                    () // do nothing
                }
            }

        }
        Ok(Some(ret))

    }
}
