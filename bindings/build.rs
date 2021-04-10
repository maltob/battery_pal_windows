fn main() { 
    windows::build!( Windows::Devices::Power::{Battery, BatteryReport}, Windows::System::Power::{BatteryStatus}, Windows::Foundation::IReference);
}