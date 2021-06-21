//! An InvocableCategoryList contains a list of InvocableCategory,
//! which are named categories that each contain a list of Invocable,
//! where each Invocable contains metadata about a command that the operating system can invoke.

//TODO: is there a better way to reference the get_config_file_path function and the InvocableCategory struct?

use crate::wsl::get_config_file_path;
use crate::wsl::inv::invocablecategory::InvocableCategory;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InvocableCategoryList {
    /// The categories field contains the list of InvocableCategory.
    pub categories: Vec<InvocableCategory>,
}

impl InvocableCategoryList {
    /// Return an InvocableCategoryList populated from a hard-coded list of categories
    /// plus the contents of $HOME/.wink.json (WSL) or $USERPROFILE/wink.json (Windows).
    pub fn get() -> InvocableCategoryList {
        let mut category_list = InvocableCategoryList { categories: Vec::new() };

        //TODO: convert to a list of names and delegate methods

        let mut screensavers = InvocableCategory::new("Screen Savers");
        screensavers.add_screensavers();
        category_list.categories.push(screensavers);

        let mut applications = InvocableCategory::new("Applications");
        applications.add_applications();
        category_list.categories.push(applications);

        let mut locations = InvocableCategory::new("Locations");
        locations.add_locations();
        category_list.categories.push(locations);

        let mut settings = InvocableCategory::new("Settings");
        settings.add_settings();
        category_list.categories.push(settings);

        let mut sysinternals = InvocableCategory::new("Sysinternals.com");
        sysinternals.add_sysinternals();
        category_list.categories.push(sysinternals);

        let mut networking = InvocableCategory::new("Networking");
        networking.add_networking();
        category_list.categories.push(networking);

        let mut privacy = InvocableCategory::new("Privacy");
        privacy.add_privacy();
        category_list.categories.push(privacy);

        let mut ease_of_access = InvocableCategory::new("Ease of Access");
        ease_of_access.add_ease_of_access();
        category_list.categories.push(ease_of_access);

        let mut security = InvocableCategory::new("Security");
        security.add_security();
        category_list.categories.push(security);

        let mut linux = InvocableCategory::new("Linux");
        linux.add_linux();
        category_list.categories.push(linux);

        let mut miscellaneous = InvocableCategory::new("Miscelaneous");
        miscellaneous.add_miscellaneous();
        category_list.categories.push(miscellaneous);

        let mut features = InvocableCategory::new("Features");
        features.add_features();
        category_list.categories.push(features);

        let mut shutdown = InvocableCategory::new("Shutdown");
        shutdown.add_shutdown();
        category_list.categories.push(shutdown);

        let mut utilities = InvocableCategory::new("Utilities");
        utilities.add_utilities();
        category_list.categories.push(utilities);

        let mut office = InvocableCategory::new("Microsoft Office");
        office.add_office();
        category_list.categories.push(office);

        let path: String = get_config_file_path("wink.json");

        if std::path::Path::new(&path).exists() {
            // if the path exists, then propagate all errors, so OK to unwrap from here
            let data = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Unable to read {0}", &path));
            let deserialized: InvocableCategoryList = serde_json::from_str(&data).unwrap();

            //TODO: replace following check to update hard-coded categories with anything from the configuration file.

            // if any category already exists, it will appear twice; report it.
            for category in deserialized.categories.iter() {
                for existing in category_list.categories.iter() {
                    if category.name == existing.name {
                        eprintln!("Category {0} defined in multiple places including {1}", category.name, path);
                    }
                }

                // if any command code exists twice within a category, report it.
                // subsequent logic will check for connflicts accross categories.
                let mut existing: Vec<&str> = vec![];

                for invocable in category.invocables.iter() {
                    if existing.contains(&invocable.command_code.as_str()) {
                        eprintln!("Command code {0} defined multiple places including {1} category of {2}", &invocable.command_code, &category.name, path);
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
                                    //TODO: enhance to error messages to show command line or at least wink command name
                                    eprintln!("Command code {0} defined for both {1} {2} and {3} {4}", invocable.command_code, category.name, invocable.command, compcat.name, compinv.command);
                                }
                            }
                        }
                    }
                }
            }
        }

        category_list
    }
}
