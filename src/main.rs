#![windows_subsystem = "windows"]
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::time::Duration;

#[derive(Default, NwgUi)]
pub struct BatteryPalWinTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("./icons/MidBatt.ico"))]
    #[nwg_events(MousePressLeftUp: [BatteryPalWinTray::show_menu], OnContextMenu: [BatteryPalWinTray::show_menu])]
    icon_mid: nwg::Icon,

    #[nwg_resource(source_file: Some("./icons/LowBatt.ico"))]
    #[nwg_events(MousePressLeftUp: [BatteryPalWinTray::show_menu], OnContextMenu: [BatteryPalWinTray::show_menu])]
    icon_low: nwg::Icon,

    #[nwg_resource(source_file: Some("./icons/HighBatt.ico"))]
    #[nwg_events(MousePressLeftUp: [BatteryPalWinTray::show_menu], OnContextMenu: [BatteryPalWinTray::show_menu])]
    icon_high: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon_mid), tip: Some("BatteryPalWin"))]
    #[nwg_events(MousePressLeftUp: [BatteryPalWinTray::show_menu], OnContextMenu: [BatteryPalWinTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [BatteryPalWinTray::exit])]
    tray_item3: nwg::MenuItem,

    #[nwg_control(parent: window, interval: Duration::from_millis(1000*2), active: true)]
    #[nwg_events(OnTimerTick: [BatteryPalWinTray::update_icon])]
    refresh_timer: nwg::AnimationTimer,
}

impl BatteryPalWinTray {

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn update_icon(&self) {
        //We don't have a battery, just cycle through icons
       self.select_random_icon();
    }

    fn select_random_icon(&self) {
        let v_icons = vec![&self.icon_high,&self.icon_low, &self.icon_mid];
       let mut rng = thread_rng();
       match v_icons.choose(&mut rng) {
           Some(icon) => {
                        self.tray.set_icon(&icon);
                        self.tray.set_tip(&"Battery Pal did not detect a battery");
                    },
           None => ()
       }
    }

}

fn main()  {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = BatteryPalWinTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}