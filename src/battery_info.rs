use bindings::{Windows::Devices::Power::{Battery, BatteryReport}, 
                Windows::System::Power::{BatteryStatus}
            };

pub struct BatteryInfo {

}

impl BatteryInfo {

    // General function to get the aggregate battery report
    fn battery_report() -> BatteryReport {
        let batt = Battery::AggregateBattery().expect("Could not find AggregateBattery! This shouldn't happen, even on systems without batteries.");
        batt.GetReport().expect("Could not retrieve battery report!")
    }

   pub fn battery_percentage() -> i32  {
        let report = BatteryInfo::battery_report();
        
        let mut charge:i32 = -1;
        if BatteryInfo::battery_present() {
                let full_charge = report.FullChargeCapacityInMilliwattHours().expect("Could not find full charge capacity!").GetInt32().expect("Could not find full charge capacity as Int32!");
                let current_charge = report.RemainingCapacityInMilliwattHours().expect("Could not find current charge capacity!").GetInt32().expect("Could not find current charge capacity as Int32!");
                
                // Don't run into divide by 0 errors
                if full_charge > 0 {
                    charge = ((current_charge as f32/full_charge as f32)*100.0).round() as i32;
                }
                
                //Don't allow charges over 100 to be reported - I am not sure if this is possible but just to be sure
                if charge > 100 {
                    charge = 100;
                }
        }
        charge  
    }


    // Checks for a battery in the battery report
   pub fn battery_present() -> bool  {
        let report = BatteryInfo::battery_report();

        match report.Status().expect("Could not get battery status.") {
            BatteryStatus::NotPresent => false,
            _ => true
        }
    }

    
}