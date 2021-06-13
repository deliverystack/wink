//! The InvocableCategory struct contains a name for the category and a list of Invocables for that category.
//! An InvocableCategoryList creates an InvocableCategory and can use any of the Add() functions in InvocableCategory
//! to add Invocables to that category.
use std::cmp::Ordering;

use crate::invocable;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct InvocableCategory {
    /// The friendly name of the category.
    pub name: String,

    /// The list of Invocables in the category.
    pub invocables: Vec<invocable::Invocable>,
}

/// For sorting a list of InvocableCategories by name
impl Eq for InvocableCategory {}

/// For sorting a list of InvocableCategories by name
impl Ord for InvocableCategory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

/// For sorting a list of InvocableCategories by name
impl PartialOrd for InvocableCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// For sorting a list of InvocableCategories by name
impl PartialEq for InvocableCategory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl InvocableCategory {
    /// Retrieve an InvocableCategory with the given name and an empty list of invocables.
    pub fn new(name: &'static str) -> InvocableCategory {
        InvocableCategory { name: name.to_string(), invocables: vec![] }
    }

    /// Add an Invocable to the list in this InvocableCategory.
    /// Write to stderr if the  command code is a duplicate withih this category.
    //TODO: this duplicates logic on file read; might as well check everything in that once place (consolidate this logic).
    pub fn add(&mut self, invocable: invocable::Invocable) {
        for compare in self.invocables.iter() {
            if compare.command_code == invocable.command_code {
                eprintln!("Command code {} overridden from {1} to {2}", compare.command_code, compare.command, invocable.command);
            }
        }

        self.invocables.push(invocable);
    }

    /// Add Microsoft Office Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_office(&mut self) {
        self.add(invocable::Invocable::exp("onenote", "onenote:", "Microsoft OneNote")); //  ONENOTE.EXE
        self.add(invocable::Invocable::bin("excel", "$pf64/Microsoft Office/root/Office16/EXCEL.EXE", "Microsoft Excel"));
        self.add(invocable::Invocable::bin("word", "$pf64/Microsoft Office/root/Office16/WINWORD.EXE", "Microsoft Word"));
        self.add(invocable::Invocable::bin("outlook", "$pf64/Microsoft Office/root/Office16/OUTLOOK.EXE", "Microsoft Outlook"));
        self.add(invocable::Invocable::bin("ppt", "$pf64/Microsoft Office/root/Office16/POWERPNT.EXE", "Microsoft PowerPoint"));
        self.add(invocable::Invocable::exp("od", "shell:::{018D5C66-4533-4307-9B53-224DE2ED1FE6}", "Microsoft OneDrive"));
        // C:\Program Files\Microsoft OneDrive\onedrive.exe
    }

    /// Add Windows locations Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_locations(&mut self) {
        self.add(invocable::Invocable::exp("portdev", "shell:::{35786D3C-B075-49b9-88DD-029876E11C01}", "Portable Devices folder"));
        self.add(invocable::Invocable::exp("thisdev", "shell:::{5b934b42-522b-4c34-bbfe-37a3ef7b9c90}", "This Device folder"));
        self.add(invocable::Invocable::exp("homegrp", "shell:::{6785BFAC-9D2D-4be5-B7E2-59937E8FB80A}", "Home Group folder"));
        self.add(invocable::Invocable::exp("commonpl", "shell:::{d34a6ca6-62c2-4c34-8a7c-14709c1ad938}", "Common Places"));
        self.add(invocable::Invocable::exp("removabl", "shell:::{a6482830-08eb-41e2-84c1-73920c2badb9}", "Removable Devices folder"));
        self.add(invocable::Invocable::exp("startup", "shell:startup", "User Startup Folder"));
        self.add(invocable::Invocable::exp("allstart", "shell:Common Startup", "Common Startup Folder"));
        self.add(invocable::Invocable::exp("pc", "shell:MyComputerFolder", "This Computer")); // shell:::{20D04FE0-3AEA-1069-A2D8-08002B30309D}
        self.add(invocable::Invocable::exp("saveloc", "ms-settings:savelocations", "Change where new content is saved"));
        self.add(invocable::Invocable::exp("ringtone", "shell:Ringtones", "")); //TODO: doc
        self.add(invocable::Invocable::exp("ringtonc", "shell:CommonRingtones", "")); //TODO: doc
        self.add(invocable::Invocable::exp("searches", "shell:Searches", "")); //TODO: doc
        self.add(invocable::Invocable::exp("expsrch", "shell:SearchHomeFolder", "")); //TODO: doc
        self.add(invocable::Invocable::exp("appdata", "shell:AppData", "")); //TODO: doc
        self.add(invocable::Invocable::exp("admtools", "shell:Common Administrative Tools", "")); //TODO: doc
        self.add(invocable::Invocable::exp("unupdate", "shell:AppUpdatesFolder", "Installed Updates/Uninstall an Update")); //TODO: doc // shell:::{d450a8a1-9568-45c7-9c0e-b4f9fb4537bd}
        self.add(invocable::Invocable::exp("burn", "shell:CD Burning", "")); //TODO: doc
        self.add(invocable::Invocable::exp("resource", "shell:ResourceDir", "")); //TODO: doc
        self.add(invocable::Invocable::exp("savegame", "shell:SavedGames", "")); //TODO: doc
        self.add(invocable::Invocable::exp("sys32", "shell:System", "")); //TODO: doc
        self.add(invocable::Invocable::exp("sys86", "shell:Systemx86", "")); //TODO: doc
        self.add(invocable::Invocable::exp("roamtile", "shell:Roaming Tiles", "%USERPROFILE%\\AppData\\Local\\Microsoft\\Windows\\RoamingTiles"));
        self.add(invocable::Invocable::exp("progf", "shell:ProgramFiles", "C:\\Program Files")); //TODO: doc
        self.add(invocable::Invocable::exp("progfc", "shell:ProgramFilesCommon", "C:\\Program Files\\Common Files"));
        self.add(invocable::Invocable::exp("progf86", "shell:ProgramFilesX86", "C:\\Program Files (x86)"));
        self.add(invocable::Invocable::exp("progfc86", "shell:ProgramFilesCommonX86", "C:\\Program Files (x86)\\Common Files"));
        self.add(invocable::Invocable::exp("public", "shell:Public", "")); //TODO: doc
        self.add(invocable::Invocable::exp("printhood", "shell:PrintHood", "")); //TODO: doc
        self.add(invocable::Invocable::exp("printrsf", "shell:PrintersFolder", "")); //TODO: doc
        self.add(invocable::Invocable::exp("programf", "shell:Programs", "")); //TODO: doc
        self.add(invocable::Invocable::exp("profilef", "shell:Profile", "")); //TODO: doc
        self.add(invocable::Invocable::exp("userpins", "shell:User Pinned", "")); //TODO: doc
        self.add(invocable::Invocable::exp("playlist", "shell:playlists", "")); //TODO: doc
        self.add(invocable::Invocable::exp("programsc", "shell:Common Programs", "Shared programs folder")); //TODO: doc
        self.add(invocable::Invocable::exp("templates", "shell:templates", "")); //TODO: doc
        self.add(invocable::Invocable::exp("sendto", "shell:sendto", "")); //TODO: doc
        self.add(invocable::Invocable::exp("freq", "shell:::{3936E9E4-D92C-4EEE-A85A-BC16D5EA0819}", "Frequently accessed folders"));
        self.add(invocable::Invocable::exp("download", "shell:Downloads", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pubdown", "shell:CommonDownloads", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pubdesk", "shell:Common Desktop", "Common desktop folder")); //TODO: doc
        self.add(invocable::Invocable::exp("desktop", "shell:Desktop", "User desktop folder")); // shell:::{00021400-0000-0000-C000-000000000046}", // error?
        self.add(invocable::Invocable::exp("apps", "shell:AppsFolder", "Applications folder")); //{4234d49b-0245-4df3-b780-3893943456e1}
        self.add(invocable::Invocable::exp("recyc", "shell:RecycleBinFolder", "Recycle bin folder")); // shell:::{645FF040-5081-101B-9F08-00AA002F954E}
        self.add(invocable::Invocable::exp("dpapikey", "shell:DpAPIKeys", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pubdocs", "shell:Common Documents", "")); //TODO: doc
        self.add(invocable::Invocable::exp("favs", "shell:Favorites", "Favorites folder")); //TODO: doc // shell:::{323CA680-C24D-4099-B94D-446DD2D7249E}
        self.add(invocable::Invocable::exp("fontsdir", "shell:Fonts", "")); //TODO: doc
        self.add(invocable::Invocable::exp("doclib", "shell:DocumentsLibrary", "")); //TODO: doc
        self.add(invocable::Invocable::exp("acctpict", "shell:AccountPictures", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pictlib", "shell:PicturesLibrary", "")); //TODO: doc
        self.add(invocable::Invocable::exp("links", "shell:Links", "")); //TODO: doc
        self.add(invocable::Invocable::exp("quick", "shell:Quick Launch", "")); //TODO: doc
        self.add(invocable::Invocable::exp("recent", "shell:recent", "")); //TODO: doc
        self.add(invocable::Invocable::exp("addnprog", "shell:AddNewProgramsFolder", "")); //TODO: doc
        self.add(invocable::Invocable::exp("chngprog", "shell:ChangeRemoveProgramsFolder", "")); //TODO: doc
        self.add(invocable::Invocable::exp("gamexp", "shell:PublicGameTasks", "")); //TODO: doc
        self.add(invocable::Invocable::exp("contacts", "shell:Contacts", "")); //TODO: doc
        self.add(invocable::Invocable::exp("cookies", "shell:Cookies", "")); //TODO: doc
        self.add(invocable::Invocable::exp("creds", "shell:CredentialManager", "")); //TODO: doc
        self.add(invocable::Invocable::exp("vidlib", "shell:VideosLibrary", "")); //TODO: doc
        self.add(invocable::Invocable::exp("libs", "shell:Libraries", "")); //TODO: doc  //shell:::{031E4825-7B94-4dc3-B131-E946B44C8DD5}
        self.add(invocable::Invocable::exp("history", "shell:History", "")); //TODO: doc
        self.add(invocable::Invocable::exp("impappsc", "shell:ImplicitAppShortcuts", "")); //TODO: doc
        self.add(invocable::Invocable::exp("crypkeys", "shell:Cryptokeys", "")); //TODO: doc
        self.add(invocable::Invocable::exp("inetcach", "shell:cache", "")); //TODO: doc
        self.add(invocable::Invocable::exp("startme", "shell:Start Menu", "")); //TODO: doc
        self.add(invocable::Invocable::exp("startall", "shell:Common Start Menu", "")); //TODO: doc
        self.add(invocable::Invocable::exp("windowsf", "shell:Windows", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pubvideo", "shell:CommonVideo", "")); //TODO: doc
        self.add(invocable::Invocable::exp("myvideo", "shell:My Video", "")); //TODO: doc
        self.add(invocable::Invocable::exp("mydocs", "shell:::{450D8FBA-AD25-11D0-98A8-0800361B1103}", "My Documents"));
        self.add(invocable::Invocable::exp("mail", "outlookmail:", ""));
        self.add(invocable::Invocable::exp("mediasrv", "shell:::{289AF617-1CC3-42A6-926C-E6A863F0E3BA}", "Media Servers"));
        self.add(invocable::Invocable::exp("msvideo", "microsoftvideo:", ""));
    }

    /// Add SysInternals utilities Invocables to the list of Invocables in this InvocableCategory.
    /// https://live.sysinternals.com/ https://docs.microsoft.com/en-us/sysinternals/
    pub fn add_sysinternals(&mut self) {
        self.add(invocable::Invocable::cmd_with("bginfo", "$syslivebginfo64.exe", "Set desktop background to system information", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("handle", "$syslivehandle64.exe", "List open file handles", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("listdlls", "$syslivelistdlls64.exe", "List processes and their DLLs", &["-accepteula"]));
        //TODO: "whoson" => Invoker::cmd(format!("{}whoson64.exe", &["-accepteula"]), // nosuch?
        self.add(invocable::Invocable::cmd_with("procexp", "$sysliveprocexp64.exe", "Process Explorer", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("pslist", "$syslivepslist64.exe", "Process lister", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("pskill", "$syslivepskill.exe", "Process killer", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("procmon", "$sysliveprocmon64.exe", "Process Monitor", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("autoruns", "$sysliveautoruns64.exe", "Identify and control startup processes", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("diskview", "$syslivediskview64.exe", "Disk space usage visualizer", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("du", "$syslivedu64.exe", "Disk usage", &["-accepteula"]));
        self.add(invocable::Invocable::cmd_with("zoomit", "$syslivezoomit63.exe", "crash it, change it, mail – upgrade it, Charge it, point it, zoom it, press it, Snap it, work it, quick – erase it...Technologic", &["-accepteula"]));
    }

    /// Add Windows settings Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_settings(&mut self) {
        self.add(invocable::Invocable::exp("srchsets", "ms-settings:cortana-windowssearch", "Windows Search (Cortana) Settings"));
        self.add(invocable::Invocable::exp("filehist", "shell:::{F6B6E965-E9B2-444B-9286-10C9152EDBC5}", "File History"));
        self.add(invocable::Invocable::exp("syncset", "ms-settings:sync", "Sync Settings...settings"));
        self.add(invocable::Invocable::exp("devices", "shell:::{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "Devices and Printers"));
        self.add(invocable::Invocable::exp("storsp", "shell:::{F942C606-0914-47AB-BE56-1321B8035096}", "Manage Storage Spaces"));
        self.add(invocable::Invocable::exp("control", "shell:ControlPanelFolder", "Control Panel (small icons)")); // shell:::{21EC2020-3AEA-1069-A2DD-08002B30309D}
        self.add(invocable::Invocable::exp("cpcats", "shell:::{26EE0668-A00A-44D7-9371-BEB064C98683}", "Control Panel (categories)"));
        self.add(invocable::Invocable::exp("fileopt", "shell:::{6DFD7C5C-2451-11d3-A299-00C04F8EF6AF}", "File Explorer settings"));
        self.add(invocable::Invocable::cmd_with("oldfonts", "control.exe", "Legacy fonts control panel", &["fonts"])); //TODO: duplicate?
        self.add(invocable::Invocable::cmd("perfopt", "SystemPropertiesPerformance.exe", "Performance Options"));
        self.add(invocable::Invocable::cmd("devman", "hdwwiz.cpl", "Device Manager control pane"));
        self.add(invocable::Invocable::cmd("timedate", "timedate.cpl", "Date and Time control panel"));
        self.add(invocable::Invocable::cmd("inet", "inetcpl.cpl", "Internet control panel"));
        self.add(invocable::Invocable::cmd("joy", "joy.cpl", "Game Controllers control panel"));
        self.add(invocable::Invocable::cmd("loc", "telephon.cpl", "Location Information control panel"));
        self.add(invocable::Invocable::cmd("mouse", "main.cpl", "Mouse control panel"));
        self.add(invocable::Invocable::exp("power", "shell:::{025A5937-A6BE-4686-A844-36FE4BEC8B6D}", "Power control panel")); // powercfg.cpl
        self.add(invocable::Invocable::cmd("sound", "mmsys.cpl", "Sound control panel"));
        self.add(invocable::Invocable::cmd("sysprop", "sysdm.cpl", "Sound Properties control panel")); // SystemPropertiesHardware.exe SystemPropertiesAdvanced.exe
        self.add(invocable::Invocable::exp("about", "ms-settings:about", "About settings")); // shell:::{BB06C0E4-D293-4f75-8A90-CB05B6477EEE}
        self.add(invocable::Invocable::exp("autoplay", "ms-settings:autoplay", "Autoplay Settings (defaults)"));
        self.add(invocable::Invocable::exp("autoplyd", "shell:::{9C60DE1E-E5FC-40f4-A487-460851A8D915}", "Autoplay Settings by Device type"));
        self.add(invocable::Invocable::exp("battery", "ms-settings:batterysaver", "Battery settings"));
        self.add(invocable::Invocable::exp("captures", "ms-settings:gaming-gamedvr", "Screen capture settings"));
        self.add(invocable::Invocable::exp("clp", "ms-settings:clipboard", "Clipboard settings"));
        self.add(invocable::Invocable::exp("condev", "ms-settings:connecteddevices", "Connected devices (bluetooth and other devices) settings"));
        self.add(invocable::Invocable::exp("crossdev", "ms-settings:crossdevice", "Shared experiences accross devices settings"));
        self.add(invocable::Invocable::exp("datause", "ms-settings:datausage", "Data usage settings"));
        self.add(invocable::Invocable::exp("default", "ms-settings:defaultapps", "Choose default applications settings")); // shell:::{2559a1f7-21d7-11d4-bdaf-00c04f60b9f0}
        self.add(invocable::Invocable::exp("devdisc", "ms-settings-connectabledevices:devicediscovery", "Connectable device discovery settings"));
        self.add(invocable::Invocable::exp("focus", "ms-settings:quiethours", "Quiet hours/focus assist settungs")); // ms-settings:quietmomentshome ms-settings:quietmomentsscheduled ms-settings:quietmomentspresentation ms-settings:quietmomentsgame
        self.add(invocable::Invocable::exp("morfonts", "shell:::{93412589-74D4-4E4E-AD0E-E0CB621440FD}", "More Font settings"));
        self.add(invocable::Invocable::exp("gamemode", "ms-settings:gaming-gamemode", "Game mode settings"));
        self.add(invocable::Invocable::exp("graphics", "ms-settings:display-advancedgraphics", "Advanced graphics settings"));
        self.add(invocable::Invocable::exp("keyboard", "ms-settings:keyboard", "Keyboard settings")); //TODO: language?
        self.add(invocable::Invocable::exp("multitsk", "ms-settings:multitasking", "Multitasking settings"));
        self.add(invocable::Invocable::exp("nags", "ms-settings:notifications", "Notification settings"));
        self.add(invocable::Invocable::exp("night", "ms-settings:nightlight", "Night light settings"));
        self.add(invocable::Invocable::exp("maps", "ms-settings:maps", "Map settings"));
        self.add(invocable::Invocable::exp("offlmaps", "ms-settings:maps-downloadmaps", "Offline map settings"));
        self.add(invocable::Invocable::exp("pen", "ms-settings:pen", "Pen input settings"));
        self.add(invocable::Invocable::exp("rdset", "ms-settings:remotedesktop", "")); //TODO: doc
        self.add(invocable::Invocable::exp("scaling", "ms-settings:display-advanced", "")); //TODO: doc
        self.add(invocable::Invocable::exp("prntscan", "ms-settings:printers", "Printer and Scanner settings"));
        self.add(invocable::Invocable::exp("setemail", "ms-settings:emailandaccounts", "Email and Accounts settings"));
        self.add(invocable::Invocable::exp("setspch", "ms-settings:speech", "")); //TODO: doc
        self.add(invocable::Invocable::exp("speech", "shell:::{58E3C745-D971-4081-9034-86E34B30836A}", "Configure your speech recognition experience")); //TODO: doc
        self.add(invocable::Invocable::exp("setstart", "ms-settings:startupapps", "")); //TODO: doc
        self.add(invocable::Invocable::exp("setvideo", "ms-settings:videoplayback", "")); //TODO: doc
        self.add(invocable::Invocable::exp("sounddev", "ms-settings:sound-devices", "")); //TODO: doc
        self.add(invocable::Invocable::exp("sounds", "ms-settings:sound", "Sound settings")); //TODO: doc
        self.add(invocable::Invocable::exp("storpol", "ms-settings:storagepolicies", "Storage Policies")); //TODO: doc
        self.add(invocable::Invocable::exp("storsens", "ms-settings:storagesense", "Storage Sense"));
        self.add(invocable::Invocable::exp("tablet", "ms-settings:tabletmode", "Tablet Mode settings"));
        self.add(invocable::Invocable::exp("themes", "ms-settings:themes", "Windows Themes settings"));
        self.add(invocable::Invocable::exp("touchpad", "ms-settings:devices-touchpad", "Touchpad settings"));
        self.add(invocable::Invocable::exp("typing", "ms-settings:typing", "Typing settings"));
        self.add(invocable::Invocable::exp("usb", "ms-settings:usb", "USB settings"));
        self.add(invocable::Invocable::exp("webapps", "ms-settings:appsforwebsites", "")); //TODO: doc
        self.add(invocable::Invocable::exp("workplc", "ms-settings:workplace", "Workplace or school settings"));
        self.add(invocable::Invocable::exp("yourinfo", "ms-settings:yourinfo", "Your Information"));
        self.add(invocable::Invocable::exp("pows", "ms-settings:powersleep", "Power and Sleep settings"));
        self.add(invocable::Invocable::exp("project", "ms-settings-displays-topology:projection", "Project (dark gray sidebar at right of screen)"));
        self.add(invocable::Invocable::exp("projectme", "ms-settings:project", "Projecting to this PC"));
        self.add(invocable::Invocable::exp("background", "ms-settings:personalization-background", "Desktop background settings"));
        self.add(invocable::Invocable::exp("colors", "ms-settings:personalization-colors", "Windows Colors settings")); // ms-settings:colors
        self.add(invocable::Invocable::exp("datetime", "ms-settings:dateandtime", "Date and time settings"));
        self.add(invocable::Invocable::exp("dev", "ms-settings:developers", "Developer settings"));
        self.add(invocable::Invocable::exp("display", "ms-settings:display", "Display settings")); // ms-settings:screenrotation // ms-settings:easeofaccess-colorfilter-adaptivecolorlink // ms-settings:easeofaccess-colorfilter-bluelightlink
        self.add(invocable::Invocable::exp("gamebar", "ms-settings:gaming-gamebar", "Game bar settings"));
        self.add(invocable::Invocable::exp("lock", "ms-settings:lockscreen", "Lock screen settings"));
        self.add(invocable::Invocable::exp("mouseset", "ms-settings:mousetouchpad", "Mouse settings"));
        self.add(invocable::Invocable::exp("personal", "shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921}", "Personalization settings")); //TODO: ms-settings:personalization is desktop background?
        self.add(invocable::Invocable::exp("taskbar", "ms-settings:taskbar", "Taskbar settings")); // shell:::{0DF44EAA-FF21-4412-828E-260A8728E7F1}
        self.add(invocable::Invocable::exp("vol", "ms-settings:apps-volume", "Volume settings"));
        self.add(invocable::Invocable::exp("activat", "ms-settings:activation", "Windows activation settings"));
        self.add(invocable::Invocable::exp("backup", "ms-settings:backup", "Backup settings"));
        self.add(invocable::Invocable::exp("recover", "ms-settings:recovery", "Recovery settings"));
        self.add(invocable::Invocable::exp("otherusr", "ms-settings:otherusers", "Other users settings"));
        self.add(invocable::Invocable::exp("findmydv", "ms-settings:findmydevice", "Find my device settings"));
        self.add(invocable::Invocable::exp("region", "ms-settings:regionformatting", "Regional formatting settings"));
        self.add(invocable::Invocable::exp("language", "ms-settings:regionlanguage", "Regional language settings")); // ms-settings:regionlanguage-languageoptions ms-settings:regionlanguage-setdisplaylanguage ms-settings:regionlanguage-adddisplaylanguage
        self.add(invocable::Invocable::exp("settings", "ms-settings:", "Settings control panel"));
        self.add(invocable::Invocable::exp("start", "ms-settings:personalization-start", "Start Menu personalization settings"));
        self.add(invocable::Invocable::exp("startfol", "ms-settings:personalization-start-places", "Personalize/choose which folders appear on the start menu"));
    }

    /// Add Windows programs and features Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_features(&mut self) {
        self.add(invocable::Invocable::exp("features", "ms-settings:appsfeatures", "Optional Apps and Features Sttings")); // optionalfeatures.exe shell:::{67718415-c450-4f3c-bf8a-b487642dc39b}
        self.add(invocable::Invocable::exp("optional", "ms-settings:optionalfeatures", "Optional Features Settings"));
        self.add(invocable::Invocable::cmd("programs", "appwiz.cpl", "Uninstall or Change a Program"));
        self.add(invocable::Invocable::cmd("defaults", "shell:::{17cd9488-1228-4b2f-88ce-4298e93e0966}", "Default Programs"));
        self.add(invocable::Invocable::exp("update", "ms-settings:windowsupdate-action", "Windows Update"));
        self.add(invocable::Invocable::exp("updateh", "ms-settings:windowsupdate-history", "Windows Update History"));
        self.add(invocable::Invocable::exp("updater", "ms-settings:windowsupdate-restartoptions", "Windows Update Restart Options Settings"));
        self.add(invocable::Invocable::exp("updateo", "ms-settings:windowsupdate-options", "Windows Update Advanced Option Settingss"));
        self.add(invocable::Invocable::exp("updateah", "ms-settings:windowsupdate-activehours", "Windows Update Active Hours Settings"));
        self.add(invocable::Invocable::exp("delivopt", "ms-settings:delivery-optimization", "Microsoft Updates Delivery Optimization Settings"));
    }

    /// Add networking components to the list of Invocables in this InvocableCategory.
    pub fn add_networking(&mut self) {
        self.add(invocable::Invocable::exp("remoteapp", "shell:::{241D7C96-F8BF-4F85-B01F-E2B043341A4B}", "RemoteApp and Desktop Connections"));
        self.add(invocable::Invocable::exp("yurphone", "ms-settings:mobile-devices", "Mobile Devices/Your Phone"));
        self.add(invocable::Invocable::exp("addphone", "ms-settings:mobile-devices-addphone-direct", "Mobile Devices/Add Phone"));
        self.add(invocable::Invocable::exp("addnetp", "shell:::{D4480A50-BA28-11d1-8E75-00C04FA31A86}", "Add Network Place"));
        self.add(invocable::Invocable::exp("netcon", "shell:ConnectionsFolder", "Network Connections folder")); // ncpa.cpl
        self.add(invocable::Invocable::exp("netshare", "shell:::{8E908FC9-BECC-40f6-915B-F4CA0E70D03D}", "Network and Sharing Center"));
        self.add(invocable::Invocable::exp("netavail", "ms-availablenetworks:", "Available networks"));
        self.add(invocable::Invocable::exp("nethood", "shell:NetHood", "Network shortcuts"));
        self.add(invocable::Invocable::exp("dialup", "ms-settings:network-dialup", "Dialup networking settings"));
        self.add(invocable::Invocable::exp("ethernet", "ms-settings:network-ethernet", "Ethernet LAN cable networking settings"));
        self.add(invocable::Invocable::exp("hotspot", "ms-settings:network-mobilehotspot", "Mobile hotspot wifi network settings"));
        self.add(invocable::Invocable::exp("proxy", "ms-settings:network-proxy", "")); //TODO: doc
        self.add(invocable::Invocable::exp("vpn", "ms-settings:network-vpn", "")); //TODO: doc
        self.add(invocable::Invocable::exp("wheelsup", "ms-settings:network-airplanemode", "")); //TODO: doc
        self.add(invocable::Invocable::exp("wifinets", "ms-settings:network-wifisettings", "")); //TODO: doc
        self.add(invocable::Invocable::exp("wifisets", "ms-settings:network-wifi", "")); //TODO: doc
        self.add(invocable::Invocable::exp("bluetoo", "ms-settings:bluetooth", "")); //TODO: doc
        self.add(invocable::Invocable::exp("netstat", "ms-settings:network", "Network status"));
    }

    /// Add Windows privacy controls Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_privacy(&mut self) {
        self.add(invocable::Invocable::exp("srchprm", "ms-settings:search-permissions", "Permissions and History"));
        self.add(invocable::Invocable::exp("privacy", "ms-settings:privacy", "Privacy settings control panel"));
        self.add(invocable::Invocable::exp("pdoc", "ms-settings:privacy-documents", "Documents privacy settings"));
        self.add(invocable::Invocable::exp("pfs", "ms-settings:privacy-broadfilesystemaccess", "File system access privacy settings"));
        self.add(invocable::Invocable::exp("miccheck", "ms-settings:privacy-microphone", "Microphone privacy settings"));
        self.add(invocable::Invocable::exp("pappdiag", "ms-settings:privacy-appdiagnostics", "App Diagnostics privacy settings"));
        self.add(invocable::Invocable::exp("pautodwn", "ms-settings:privacy-automaticfiledownloads", "Automatic File Downloads privacy settings"));
        self.add(invocable::Invocable::exp("pbackapp", "ms-settings:privacy-backgroundapps", "Background Apps privacy settings"));
        self.add(invocable::Invocable::exp("pcalls", "ms-settings:privacy-phonecalls", "Phone Calls privacy settings"));
        self.add(invocable::Invocable::exp("pcals", "ms-settings:privacy-calendar", "Calendar privacy settings"));
        self.add(invocable::Invocable::exp("pcallh", "ms-settings:privacy-callhistory", "Call History privacy settings"));
        self.add(invocable::Invocable::exp("pcam", "ms-settings:privacy-webcam", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pcontact", "ms-settings:privacy-contacts", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pdev", "ms-settings:privacy-customdevices", "Custom/Other Devices privacy settings")); //TODO: doc
        self.add(invocable::Invocable::exp("pdocs", "ms-settings:privacy-documents", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pemail", "ms-settings:privacy-email", "Email privacy settings"));
        self.add(invocable::Invocable::exp("pfeed", "ms-settings:privacy-feedback", "")); //TODO: doc
        self.add(invocable::Invocable::exp("phist", "ms-settings:privacy-activityhistory", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pmsg", "ms-settings:privacy-messaging", "Messaging privacy settings"));
        self.add(invocable::Invocable::exp("pmyinfo", "ms-settings:privacy-accountinfo", "Accoung Information privacy settings")); //TODO: doc
        self.add(invocable::Invocable::exp("pnags", "ms-settings:privacy-notifications", "Notifications privacy settings")); //TODO: doc
        self.add(invocable::Invocable::exp("ppics", "ms-settings:privacy-pictures", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pradios", "ms-settings:privacy-radios", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pspeech", "ms-settings:privacy-speech", "")); //TODO: doc
        self.add(invocable::Invocable::exp("ptasks", "ms-settings:privacy-tasks", "")); //TODO: doc
        self.add(invocable::Invocable::exp("ptype", "ms-settings:privacy-speechtyping", "")); //TODO: doc
        self.add(invocable::Invocable::exp("pvoicea", "ms-settings:privacy-voiceactivation", "")); //TODO: doc
        self.add(invocable::Invocable::exp("stalkme", "ms-settings:privacy-location", "Location privacy settings"));
    }

    /// Add shutdown commands to the list of Invocables in this InvocableCategory.
    pub fn add_shutdown(&mut self) {
        self.add(invocable::Invocable::bin_with("boot", "shutdown.exe", "Reboot", &["/r"])); // "/t", "30"]), //reboot in 30 seconds unless shutdown.exe /a
        self.add(invocable::Invocable::bin_with("bootopt", "shutdown.exe", "Reboot to boot options", &["/r", "/o"])); //"/t", "30"]), // reboot to boot options in 30 seconds unless shutdown.exe /a
        self.add(invocable::Invocable::bin_with("down", "shutdown.exe", "Shut down", &["/s"])); // "/t", "30"]), // shut down in 30 seconds unless shutdown.exe /a
        self.add(invocable::Invocable::bin_with("firmware", "shutdown.exe", "Reboot to firmware", &["/r", "/fw"])); // "/t", "30"]), // reboot to formware in 30 seconds unless shutdown.exe /a
        self.add(invocable::Invocable::bin_with("hyb", "shutdown.exe", "Hybernate", &["/h"])); // err, "/t", "30"]), // hybernate
        self.add(invocable::Invocable::bin_with("logoff", "shutdown.exe", "Log off", &["/l"])); // "/t", "30"]), // logoff in 30 seconds unless shutdown.exe /a
        self.add(invocable::Invocable::bin_with("shutui", "shutdown.exe", "Shutdown UI", &["/i"]));
        // reboot to boot options in 30 seconds unless shutdown.exe /a         //TODO: new/doc
    }

    /// Add Windows Ease of Access settings Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_ease_of_access(&mut self) {
        self.add(invocable::Invocable::exp("eatcur", "ms-settings:easeofaccess-cursor", "Ease of access text cursor settings"));
        self.add(invocable::Invocable::exp("eamag", "ms-settings:easeofaccess-magnifier", "Ease of access magnifier"));
        self.add(invocable::Invocable::exp("eacolflt", "ms-settings:easeofaccess-colorfilter", "Ease of access color filter"));
        self.add(invocable::Invocable::exp("eahighc", "ms-settings:easeofaccess-highcontrast", "Ease of access high contrast"));
        self.add(invocable::Invocable::exp("eaeyec", "ms-settings:easeofaccess-eyecontrol", "Ease of access eye control"));
        self.add(invocable::Invocable::exp("eamouse", "ms-settings:easeofaccess-mouse", "Ease of access mouse settings"));
        self.add(invocable::Invocable::exp("eakeyb", "ms-settings:easeofaccess-keyboard", "Ease of access keyboard settings"));
        self.add(invocable::Invocable::exp("easpeech", "ms-settings:easeofaccess-speechrecognition", "Ease of access speech recognition settings"));
        self.add(invocable::Invocable::exp("eaaudio", "ms-settings:easeofaccess-audio", "Ease of access audio settings"));
        self.add(invocable::Invocable::exp("eadisp", "ms-settings:easeofaccess-display", "Ease of access display settings"));
        self.add(invocable::Invocable::exp("eanar", "ms-settings:easeofaccess-narrator", "Ease of access narrator settings")); // ms-settings:easeofaccess-narrator-isautostartenabled
        self.add(invocable::Invocable::exp("ease", "shell:::{D555645E-D4F8-4c29-A827-D93C859C4F2A}", "Ease of Access Settings")); // "control.exe",  &["access.cpl"]));
        self.add(invocable::Invocable::exp("captions", "ms-settings:easeofaccess-closedcaptioning", "Ease of access closed captioning settings"));
    }

    /// Add security-related Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_security(&mut self) {
        self.add(invocable::Invocable::exp("user", "shell:::{60632754-c523-4b62-b45c-4172da012619}", "User Accounts"));
        self.add(invocable::Invocable::cmd_with("userpass", "control.exe", "", &["userpasswords2"]));
        self.add(invocable::Invocable::cmd("psr", "psr.exe", "Password Safe Repository"));
        self.add(invocable::Invocable::exp("syscerts", "shell:SystemCertificates", "System Security Certificates"));
        self.add(invocable::Invocable::cmd_with("eup", "rundll32.exe", "Edit User Profiles", &["sysdm.cpl,EditUserProfiles"]));
        self.add(invocable::Invocable::cmd("secman", "wscui.cpl", "Security and Maintenance control panel"));
        self.add(invocable::Invocable::cmd("firewall", "firewall.cpl", "Firewall security settings"));
        self.add(invocable::Invocable::cmd("advsec", "wf.msc", "Advanced firewall security settings"));
        self.add(invocable::Invocable::exp("facesign", "ms-settings:signinoptions-launchfaceenrollment", "Face recognition security settings"));
        self.add(invocable::Invocable::exp("fingsign", "ms-settings:signinoptions-launchfingerprintenrollment", "Fingerprint recognition security settings"));
        self.add(invocable::Invocable::exp("keysign", "ms-settings:signinoptions-launchsecuritykeyenrollment", "Security key (USB) security settings"));
        self.add(invocable::Invocable::exp("upsign", "ms-settings:signinoptions-dynamiclock", "Security dynamic lock settings"));
        self.add(invocable::Invocable::exp("signin", "ms-settings:signinoptions", "Security sign-in settings"));
        self.add(invocable::Invocable::exp("seccntr", "windowsdefender:", "Windows Security Center/Security at a Glance"));
        self.add(invocable::Invocable::exp("winsec", "ms-settings:windowsdefender", "Windows Security Settings"));
        self.add(invocable::Invocable::exp("bitlock", "shell:::{D9EF8727-CAC2-4e60-809E-86F80A666C91}", "Bitlocker Drive Encryption"));
        self.add(invocable::Invocable::cmd("authman", "azman.msc", "Security Authorization Manager"));
        self.add(invocable::Invocable::cmd("certmgr", "certmgr.msc", "Security Certificate Manager"));
        self.add(invocable::Invocable::cmd("useracts", "netplwiz.exe", "Security User Accounts"));
        self.add(invocable::Invocable::exp("users", "shell:userprofiles", "User Profiles"));
        self.add(invocable::Invocable::cmd("uac", "UserAccountControlSettings.exe", ""));
    }

    /// Add Linux Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_linux(&mut self) {
        self.add(invocable::Invocable::sh("wince", "/home/jw/bin/wince", "Run the shell script that recompiles this program"));
        self.add(invocable::Invocable::sh("bash", "", "Run the Unix command specified on the command line"));
        self.add(invocable::Invocable::sh("gimp", "/usr/bin/gimp", "gimp (image manipulation)"));
        self.add(invocable::Invocable::sh("microsoft-edge", "/usr/bin/microsoft-edge-dev", "microsoft-edge (brower)"));
        self.add(invocable::Invocable::sh("google-chrome", "/usr/bin/google-chrome", "google-chrome (browser)"));
        self.add(invocable::Invocable::sh("gedit", "/usr/bin/gedit", "gedit (graphical editor)"));
        self.add(invocable::Invocable::sh("xlogo", "/usr/bin/xlogo", "xlogo (visual X logo)"));
        self.add(invocable::Invocable::sh("xmore", "/usr/bin/xmore", "xmore (read-only text UI)"));
        self.add(invocable::Invocable::sh("xgc", "/usr/bin/xgc", "xgc (graphics demo)"));
        self.add(invocable::Invocable::sh("xman", "/usr/bin/xman", "xman (man pages)"));
        self.add(invocable::Invocable::sh("xcalc", "/usr/bin/xcalc", "xcalc (calculator)"));
        self.add(invocable::Invocable::sh("xeyes", "/usr/bin/xeyes", "xeyes (visual eyeballs)"));
        self.add(invocable::Invocable::sh("xclock", "/usr/bin/xclock", "xclock (visual clock)"));
        self.add(invocable::Invocable::sh("lvlc", "/usr/bin/lvlc", "Linux VLC (media player)"));
        self.add(invocable::Invocable::sh("nautilus", "/usr/bin/nautilus", "nautilus (file browser)"));
    }

    /// Add miscelaneous Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_miscellaneous(&mut self) {
        self.add(invocable::Invocable::exp("insider", "ms-settings:windowsinsider", "Microsoft Windows Insider Program"));
        self.add(invocable::Invocable::exp("wintab", "shell:::{3080F90E-D7AD-11D9-BD98-0000947B0257}", "Switch windows (Windows+Tab)"));
        self.add(invocable::Invocable::exp("showd", "shell:::{3080F90D-D7AD-11D9-BD98-0000947B0257}", "Show Windows desktop"));
        self.add(invocable::Invocable::exp("trouble", "ms-settings:troubleshoot", "Troubleshooting Windows"));
        self.add(invocable::Invocable::cmd("quickass", "quickassist.exe", "Windows Quick Assist"));
    }

    /// Add Windows utilities Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_utilities(&mut self) {
        self.add(invocable::Invocable::exp("run", "shell:::{2559a1f3-21d7-11d4-bdaf-00c04f60b9f0}", "Windows Run Dialog"));
        self.add(invocable::Invocable::cmd("lpksetup", "lpksetup.exe", "Language Pack Setup"));
        self.add(invocable::Invocable::cmd("msinfo32", "msinfo32.exe", "System Information"));
        self.add(invocable::Invocable::cmd("verifier", "verifier.exe", "Driver Verifier Manager"));
        self.add(invocable::Invocable::cmd("iexplore", "$pf64/Internet Explorer/iexplore.exe", "Internet Explorer"));
        self.add(invocable::Invocable::cmd_with("pwrd", "rundll32.exe", "Steps Recorder", &["keymgr.dll,PRShowSaveWizardExW"]));
        self.add(invocable::Invocable::cmd("tpm", "tpminit.exe", "Trusted Platform Module"));
        self.add(invocable::Invocable::cmd("odbcconf", "odbcconf.exe", ""));
        self.add(invocable::Invocable::cmd("wmimgmt", "wmimgmt.msc", ""));
        self.add(invocable::Invocable::cmd("fsquirt", "fsquirt.exe", "Bluetooth File Transfer"));
        self.add(invocable::Invocable::cmd_with("wintools", "control.exe", "Windows Tools", &["admintools"])); //shell:::{D20EA4E1-3957-11d2-A40B-0C5020524153}
        self.add(invocable::Invocable::bin("charmap", "charmap.exe", "Character Map"));
        self.add(invocable::Invocable::cmd("cttune", "cttune.exe", "Clear Type Text Tuner"));
        self.add(invocable::Invocable::cmd("colorcpl", "colorcpl", "Color Management"));
        self.add(invocable::Invocable::cmd("compmgmt", "compmgmt.msc", "Computer Management"));
        self.add(invocable::Invocable::cmd_with("editenv", "rundll32.exe", "Edit environment variables", &["sysdm.cpl,EditEnvironmentVariables"]));
        self.add(invocable::Invocable::cmd("comserv", "dcomcnfg.exe", "Component Services Configuration"));
        self.add(invocable::Invocable::cmd("printui", "printui.exe", "Print User Interface"));
        self.add(invocable::Invocable::cmd("eudcedit", "eudcedit.exe", "Private Character Editor"));
        self.add(invocable::Invocable::cmd("osk", "osk.exe", "On-screen keyboard"));
        self.add(invocable::Invocable::bin("psise", "powershell_ise.exe", "PowerShell Integrated Scripting Environment (ISE)"));
        self.add(invocable::Invocable::bin("winver", "winver.exe", "Windows Version"));
        self.add(invocable::Invocable::bin("cdinfo", "$pf64/CrystalDiskInfo/DiskInfo64.exe", "Crystal Disk Info"));
        self.add(invocable::Invocable::exp("mobility", "shell:::{5ea4f148-308c-46d7-98a9-49041b1dd468}", "Windows Mobility Center")); // mblctr.exe
        self.add(invocable::Invocable::cmd("backup7", "sdclt.exe", "Windows 7 backup"));
        self.add(invocable::Invocable::cmd("chkdsk", "chkdsk.exe", "Check hard disk for errors and issues"));
        self.add(invocable::Invocable::cmd("cmd", "", "cmd.exe (see above)"));
        self.add(invocable::Invocable::exp("exp", "", "explorer.exe (see above)"));
        self.add(invocable::Invocable::cmd("env", "set", "Show Windows environment"));
        self.add(invocable::Invocable::exp("support", "ms-contact-support:", "Microsoft support"));
        self.add(invocable::Invocable::exp("movies", "mswindowsvideo:", "Microsoft Windows Video (Movies and TV)"));
        self.add(invocable::Invocable::exp("groove", "mswindowsmusic:", "Microsoft Groove Music"));
        self.add(invocable::Invocable::exp("bingmaps", "bingmaps:", "Bing Maps"));
        self.add(invocable::Invocable::exp("bingnews", "bingnews:", "Bing News"));
        self.add(invocable::Invocable::exp("msact", "ms-actioncenter:", "Windows Action Center (right dark gray sidebar)"));
        self.add(invocable::Invocable::exp("fam", "ms-wpc:", "Ask for permission (family)"));
        self.add(invocable::Invocable::bin("isoburn", "isoburn.exe", "ISO disk burner"));
        self.add(invocable::Invocable::bin("notepad", "notepad.exe", "Windows Notepad"));
        self.add(invocable::Invocable::bin("wordpad", "pf86/Windows NT/Accessories/wordpad.exe", "Windows Wordpad"));
        self.add(invocable::Invocable::bin("mp", "pf64/windows media player/wmplayer.exe", "Windows Media Player"));
        self.add(invocable::Invocable::exp("sync", "shell:::{9C73F5E5-7AE7-4E32-A8E8-8D23B85255BF}", "Sync Center"));
        self.add(invocable::Invocable::bin("paint", "paint.exe", "Windows Paint"));
        self.add(invocable::Invocable::bin("faxscan", "wfs.exe", "Windows Fax and Scan"));
        self.add(invocable::Invocable::bin("scan", "wiaacmgr.exe", "Scan"));
        self.add(invocable::Invocable::bin("rd", "mstsc.exe", "Remote Desktop Client"));
        self.add(invocable::Invocable::bin("msdt", "msdt.exe", "Microsoft Support Diagnostics Tool"));
        self.add(invocable::Invocable::exp("soundrec", "shell:appsFolder\\Microsoft.WindowsSoundRecorder_8wekyb3d8bbwe!App", "Sound Recorder")); // TODO: wrong
        self.add(invocable::Invocable::exp("sticky", "shell:appsFolder\\Microsoft.MicrosoftStickyNotes_8wekyb3d8bbwe!App", "Sticky Notes")); // TODO: wrong
        self.add(invocable::Invocable::exp("calc", "calculator:", "Calculator"));
        self.add(invocable::Invocable::exp("clock", "ms-clock:", "Clock"));
        self.add(invocable::Invocable::exp("cam", "microsoft.windows.camera:", "Camera"));
        self.add(invocable::Invocable::exp("cal", "outlookcal:", "Calendar"));
        self.add(invocable::Invocable::exp("paint3d", "ms-paint:", "Paint3D"));
        self.add(invocable::Invocable::exp("people", "ms-people:", "People"));
        self.add(invocable::Invocable::exp("photos", "ms-photos:", "Photos and Video Editor"));
        self.add(invocable::Invocable::exp("sclip", "ms-screenclip:", "Screen capture (Windows+Shift+S)"));
        self.add(invocable::Invocable::exp("ssketch", "ms-ScreenSketch:", "Snip and sketch"));
        self.add(invocable::Invocable::exp("store", "ms-windows-store:", "Microsoft store"));
        self.add(invocable::Invocable::exp("tips", "ms-get-started:", "Windows tips / getting started"));
        self.add(invocable::Invocable::exp("sol", "xboxliveapp-1297287741:", "Solitare"));
        self.add(invocable::Invocable::cmd("remoteas", "msra.exe", "Windows Remote Assistance"));
        self.add(invocable::Invocable::cmd("wusa", "wusa.exe", "Windows Update Standalone Installer"));
        self.add(invocable::Invocable::cmd("perfmon", "perfmon.msc", "Performance Monitor"));
        self.add(invocable::Invocable::cmd("hdwwiz", "hdwwiz.exe", "Add Hardware Wizard")); //TODO: doc
        self.add(invocable::Invocable::cmd("dialer", "dialer.exe", "")); //TODO: doc
        self.add(invocable::Invocable::cmd("diskpart", "diskpart.exe", "Disk partitioner")); //TODO: doc
        self.add(invocable::Invocable::cmd("magnify", "magnify.exe", "Screen wink ")); //TODO: doc
        self.add(invocable::Invocable::cmd("mdsched", "mdsched.exe", "Windows Memory Diagnostics"));
        self.add(invocable::Invocable::cmd("msconfig", "msconfig.exe", "")); //TODO: doc
        self.add(invocable::Invocable::cmd("recdisc", "recdisc.exe", "Create a system repair disk"));
        self.add(invocable::Invocable::cmd("restore", "rstrui.exe", "Restore system files and settings"));
        self.add(invocable::Invocable::cmd("sndvol", "sndvol.exe", "Sound and Volume")); //TODO: doc
        self.add(invocable::Invocable::cmd("taskmgr", "taskmgr.exe", "Windows Task Manager"));
        self.add(invocable::Invocable::cmd("taskschd", "taskschd.msc", "Windows Task Scheduler"));
        self.add(invocable::Invocable::bin("dvdplay", "dvdplay.exe", "DVD player (Windows Media Player)"));
        self.add(invocable::Invocable::cmd("eventvwr", "eventwvr.msc", "Windows Event Viewer"));
        self.add(invocable::Invocable::cmd("regedt32", "regedt32.exe", "Windows Registry Editor"));
        self.add(invocable::Invocable::cmd("resmon", "resmon.exe", "Windows Resource Monitor"));
        self.add(invocable::Invocable::cmd("services", "services.msc", "Windows Services"));
    }

    /// Add various Windows application Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_applications(&mut self) {
        self.add(invocable::Invocable::bin("zoom", "$userpath/AppData/Roaming/Zoom/bin/Zoom.exe", "Zoom"));
        self.add(invocable::Invocable::cmd_with("killzoom", "taskkill.exe", "Kill Zoom", &["/im", "zoom.exe"]));
        self.add(invocable::Invocable::bin("ransack", "$pf64/Mythicsoft/Agent Ransack/AgentRansack.exe", "Mozilla Thunderbird email client"));
        self.add(invocable::Invocable::bin("email", "shell:::{2559a1f5-21d7-11d4-bdaf-00c04f60b9f0}", "Default email program"));
        self.add(invocable::Invocable::bin("slack", "$userpath/AppData/Local/slack/slack.exe", "Slack"));
        self.add(invocable::Invocable::bin("sub", "$pf64/Sublime Text 3/sublime_text.exe", "Sublime Text Editor"));
        self.add(invocable::Invocable::bin("tb", "$pf86/Mozilla Thunderbird/thunderbird.exe", "Mozilla Thunderbird email client"));
        self.add(invocable::Invocable::bin("flp", "$pf64/Mythicsoft/FileLocator Pro/FileLocatorPro.exe", "Agent Ransack file search tool"));
        self.add(invocable::Invocable::bin("7z", "$pf64/7-Zip/7zFM.exe", "7-Zip compressed file manager"));
        self.add(invocable::Invocable::bin("irfan", "$pf64/IrfanView/i_view64.exe", "IfranView Media Viewer"));
        self.add(invocable::Invocable::bin("audacity", "$pf86/Audacity/audacity.exe", "Audacity audio file editor"));
        self.add(invocable::Invocable::bin("deskpins", "$pf86/DeskPins/deskpins.exe", "DeskPins"));
        self.add(invocable::Invocable::bin("firefox", "$pf64/Mozilla Firefox/firefox.exe", "Mozilla Firefox browser"));
        self.add(invocable::Invocable::bin("foobar", "$pf86/foobar2000/foobar2000.exe", "Foobar2000 music player"));
        self.add(invocable::Invocable::bin("linqpad", "$pf64/LINQPad6/LINQPad6.exe", "LINQPad for C#"));
        self.add(invocable::Invocable::bin("vlc", "$pf86/VideoLAN/VLC/vlc.exe", "VLC Media Player"));
        self.add(invocable::Invocable::bin("winmerge", "$pf86/WinMerge/WinMergeU.exe", "WinMerge file and directory comparison tool"));
        self.add(invocable::Invocable::bkg("dotpeek", "$userpath/AppData/Local/JetBrains/Installations/dotPeek201/dotPeek64.exe", "JetBrains dotPeek .NET disassembler"));
        self.add(invocable::Invocable::bin_with("teams", "$userpath/AppData/Local/Microsoft/Teams/Update.exe", "Microsoft Teams", &["--processStart", "Teams.exe"]));
        self.add(invocable::Invocable::bin("vs", "$pf86/Microsoft Visual Studio/2019/Community/Common7/IDE/devenv.exe", "Microsoft Visual Studio"));
        self.add(invocable::Invocable::bin("vscode", "$userpath/AppData/Local/Programs/Microsoft VS Code/Code.exe", "Microsoft Visual Studio Code"));
        self.add(invocable::Invocable::bin("rider", "$pf64/JetBrains/JetBrains Rider 2021.1.2/bin/rider64.exe", "JetBrains Rider IDE"));
        self.add(invocable::Invocable::bin_with("edge", "$pf86/Microsoft/Edge/Application/msedge.exe", "Microsoft Edge", &["--inprivate", "--ash-force-desktop", "--disable-background-mode", "--disable-preconnect", "--new-window", "--dns-prefetch-disable", "--no-pings", "--process-per-tab", "--no-referrers", "--start-maximized"]));
        self.add(invocable::Invocable::bin_with(
            "trackme",
            "$pf86/Microsoft/Edge/Application/msedge.exe",
            "Microsoft Edge",
            &[
                //                "--inprivate",
                "--ash-force-desktop",
                "--disable-background-mode",
                "--disable-preconnect",
                "--new-window",
                "--dns-prefetch-disable",
                "--no-pings",
                "--process-per-tab",
                "--no-referrers",
                "--start-maximized",
            ],
        ));
    }
}

//TODO: shell:::{7b81be6a-ce2b-4676-a29e-eb907a5126c5}", // ms-settings:network-status
//TODO:        self.add(invocable::Invocable::exp("eacur", "ms-settings:easeofaccess-cursorandpointersize", "Ease of access cursor and pointer size")); //TODO: fail
//TODO:        self.add(invocable::Invocable::exp("eapoint", "ms-settings:easeofaccess-MousePointer", "Ease of access mouse pointer settings")); //TODO: fail
