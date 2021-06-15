//! The InvocableCategoryList struct contains a list of InvocableCategories that each contain a list of Invocables.

use std::env;

use crate::file;
use crate::invocablecategory;

// a list of categories of invocable features
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InvocableCategoryList {
    pub categories: Vec<invocablecategory::InvocableCategory>,
}

impl InvocableCategoryList {
    /// Return an InvocableCategoryList populated from a hard-coded list of categories
    /// plus the contents of $HOME/.wink.json or $USERPROFILE/wink.json.
    /// Write messages about any category and code conflicts to standard error.
    pub fn get() -> InvocableCategoryList {
        let mut category_list = InvocableCategoryList { categories: Vec::new() };

        let mut applications = invocablecategory::InvocableCategory::new("Applications");
        applications.add_applications();
        category_list.categories.push(applications);

        let mut locations = invocablecategory::InvocableCategory::new("Locations");
        locations.add_locations();
        category_list.categories.push(locations);

        let mut settings = invocablecategory::InvocableCategory::new("Settings");
        settings.add_settings();
        category_list.categories.push(settings);

        let mut sysinternals = invocablecategory::InvocableCategory::new("Sysinternals.com");
        sysinternals.add_sysinternals();
        category_list.categories.push(sysinternals);

        let mut networking = invocablecategory::InvocableCategory::new("Networking");
        networking.add_networking();
        category_list.categories.push(networking);

        let mut privacy = invocablecategory::InvocableCategory::new("Privacy");
        privacy.add_privacy();
        category_list.categories.push(privacy);

        let mut ease_of_access = invocablecategory::InvocableCategory::new("Ease of Access");
        ease_of_access.add_ease_of_access();
        category_list.categories.push(ease_of_access);

        let mut security = invocablecategory::InvocableCategory::new("Security");
        security.add_security();
        category_list.categories.push(security);

        let mut linux = invocablecategory::InvocableCategory::new("Linux");
        linux.add_linux();
        category_list.categories.push(linux);

        let mut miscellaneous = invocablecategory::InvocableCategory::new("Miscelaneous");
        miscellaneous.add_miscellaneous();
        category_list.categories.push(miscellaneous);

        let mut features = invocablecategory::InvocableCategory::new("Features");
        features.add_features();
        category_list.categories.push(features);

        let mut shutdown = invocablecategory::InvocableCategory::new("Shutdown");
        shutdown.add_shutdown();
        category_list.categories.push(shutdown);

        let mut utilities = invocablecategory::InvocableCategory::new("Utilities");
        utilities.add_utilities();
        category_list.categories.push(utilities);

        let mut office = invocablecategory::InvocableCategory::new("Microsoft Office");
        office.add_office();
        category_list.categories.push(office);

        // on Unix, look for $HOME/.wink.json
        // on Windows, look for %USERPROFILE%/wink.json
        let file: String = match env::var("HOME") {
            Ok(home) => home + "/.wink.json",
            Err(_home) => match env::var("USERPROFILE") {
                Ok(val) => (val + "/wink.json"),
                Err(_val) => {
                    //                    eprintln!("Couldn't get HOME or USERPROFILE environment variable : {}", val);
                    String::new() // probably a better way to prevent attempted file reading
                }
            },
        };

        // if the file exists, read it into a temporary InvocableCategoryList
        match file::read_file(&file) {
            Ok(contents) => {
                let deserialized: InvocableCategoryList = serde_json::from_str(&contents).unwrap();

                //TODO: replace following check to update existing categories with logic something like this
                /*
                for mut category in deserialized.categories.iter() {
                    for existing in category_list.categories.iter() {
                        if category.name == existing.name {
                            for invocable in category.invocables.iter() {
                                existing.add(invocable.to_owned());
                            }
                        } else {
                            category_list.categories.add(category.to_owned());
                        }
                    }
                }*/

                // if any category already exists, it will appear twice; report it.
                for category in deserialized.categories.iter() {
                    for existing in category_list.categories.iter() {
                        if category.name == existing.name {
                            eprintln!("Category {0} defined in multiple places including {1}", category.name, file);
                        }
                    }

                    // if any command code exists twice within a category, report it.
                    // subsequent logic will check for connflicts accross categories.
                    let mut existing: Vec<&str> = vec![];

                    for invocable in category.invocables.iter() {
                        if existing.contains(&invocable.command_code.as_str()) {
                            eprintln!("Command code {0} defined multiple places including {1} category of {2}", &invocable.command_code, &category.name, file);
                        } else {
                            existing.push(&invocable.command_code);
                        }
                    }

                    // add the deserialized list to the list in memory
                    category_list.categories.push(category.to_owned());
                }

                // report duplicate command codes
                for category in category_list.categories.iter() {
                    for invocable in category.invocables.iter() {
                        for compcat in category_list.categories.iter() {
                            if compcat.name != category.name {
                                for compinv in compcat.invocables.iter() {
                                    if invocable.command_code == compinv.command_code {
                                        //TODO: function for error messages to show command name
                                        eprintln!("Command code {0} defined for both {1} {2} and {3} {4}", invocable.command_code, category.name, invocable.command, compcat.name, compinv.command);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            //            Err(err) => eprintln!("No custom configuration in {0} : {1}", file, err),
            Err(_err) => {}
        }

        category_list
    }
}
