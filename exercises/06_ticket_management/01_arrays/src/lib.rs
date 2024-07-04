//  Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.
use std::collections::HashMap;
pub struct WeekTemperatures {
    temperatures: HashMap<Weekday, Option<i32>>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        let mut new_week_temps = HashMap::new();
        new_week_temps.insert(Weekday::Monday, None);
        new_week_temps.insert(Weekday::Tuesday, None);
        new_week_temps.insert(Weekday::Wednesday, None);
        new_week_temps.insert(Weekday::Thursday, None);
        new_week_temps.insert(Weekday::Friday, None);
        new_week_temps.insert(Weekday::Saturday, None);
        new_week_temps.insert(Weekday::Sunday, None);

        return WeekTemperatures {
            temperatures: new_week_temps,
        };
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        return self.temperatures.get(&day).cloned().unwrap_or(None);
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        self.temperatures.insert(day, Some(temperature));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
