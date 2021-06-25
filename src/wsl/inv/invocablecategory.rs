//! An InvocableCategory has a name and a list of Invocables.

//TODO: is there a better way to reference Invocable?

use crate::wsl::inv::invocable::Invocable;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]

pub struct InvocableCategory {
    /// The friendly name of the category.
    pub name: String,

    /// The list of Invocables in the category.
    pub invocables: Vec<Invocable>,
}

/// For sorting a list of InvocableCategories by name
impl Eq for InvocableCategory {}

/// For sorting a list of InvocableCategories by name
impl Ord for InvocableCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

/// For sorting a list of InvocableCategories by name
impl PartialOrd for InvocableCategory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
    pub fn add(&mut self, invocable: Invocable) {
        for compare in self.invocables.iter() {
            if compare.command_code == invocable.command_code {
                eprintln!("Command code {} overridden from {1} to {2}", compare.command_code, compare.command, invocable.command);
            }
        }

        self.invocables.push(invocable);
    }

    /// Add Microsoft Office Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_office(&mut self) {
        self.add(Invocable::exp("onenote", "onenote:", "Microsoft OneNote")); //  ONENOTE.EXE
        self.add(Invocable::bin("excel", "$pf64/Microsoft Office/root/Office16/EXCEL.EXE", "Microsoft Excel"));
        self.add(Invocable::bin("word", "$pf64/Microsoft Office/root/Office16/WINWORD.EXE", "Microsoft Word"));
        self.add(Invocable::bin("outlook", "$pf64/Microsoft Office/root/Office16/OUTLOOK.EXE", "Microsoft Outlook"));
        self.add(Invocable::bin_with("oa", "$pf64/Microsoft Office/root/Office16/OUTLOOK.EXE", "Microsoft Outlook Attach File <path>", &["/a"]));
        self.add(Invocable::bin_with("oc", "$pf64/Microsoft Office/root/Office16/OUTLOOK.EXE", "Microsoft Outlook Compose", &["/c", "imp.note"]));
        self.add(Invocable::bin_with("om", "$pf64/Microsoft Office/root/Office16/OUTLOOK.EXE", "Microsoft Outlook Compose To <email>", &["/c", "imp.note", "/m"]));
        self.add(Invocable::bin("ppt", "$pf64/Microsoft Office/root/Office16/POWERPNT.EXE", "Microsoft PowerPoint"));
        self.add(Invocable::exp("od", "shell:::{018D5C66-4533-4307-9B53-224DE2ED1FE6}", "Microsoft OneDrive"));
        // C:\Program Files\Microsoft OneDrive\onedrive.exe
    }

    /// Add Windows locations Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_locations(&mut self) {
        self.add(Invocable::cmd("fsmgmt", "fsmgmt.exe", "Shared Folders"));
        self.add(Invocable::exp("portdev", "shell:::{35786D3C-B075-49b9-88DD-029876E11C01}", "Portable Devices folder"));
        self.add(Invocable::exp("thisdev", "shell:::{5b934b42-522b-4c34-bbfe-37a3ef7b9c90}", "This Device folder"));
        self.add(Invocable::exp("homegrp", "shell:::{6785BFAC-9D2D-4be5-B7E2-59937E8FB80A}", "Home Group folder"));
        self.add(Invocable::exp("commonpl", "shell:::{d34a6ca6-62c2-4c34-8a7c-14709c1ad938}", "Common Places"));
        self.add(Invocable::exp("removabl", "shell:::{a6482830-08eb-41e2-84c1-73920c2badb9}", "Removable Devices folder"));
        self.add(Invocable::exp("startup", "shell:startup", "User Startup Folder"));
        self.add(Invocable::exp("allstart", "shell:Common Startup", "Common Startup Folder"));
        self.add(Invocable::exp("pc", "shell:MyComputerFolder", "This Computer")); // shell:::{20D04FE0-3AEA-1069-A2D8-08002B30309D}
        self.add(Invocable::exp("saveloc", "ms-settings:savelocations", "Change where new content is saved"));
        self.add(Invocable::exp("ringtone", "shell:Ringtones", "")); //TODO: doc
        self.add(Invocable::exp("ringtonc", "shell:CommonRingtones", "")); //TODO: doc
        self.add(Invocable::exp("searches", "shell:Searches", "")); //TODO: doc
        self.add(Invocable::exp("expsrch", "shell:SearchHomeFolder", "")); //TODO: doc
        self.add(Invocable::exp("appdata", "shell:AppData", "")); //TODO: doc
        self.add(Invocable::exp("admtools", "shell:Common Administrative Tools", "")); //TODO: doc
        self.add(Invocable::exp("unupdate", "shell:AppUpdatesFolder", "Installed Updates/Uninstall an Update")); //TODO: doc // shell:::{d450a8a1-9568-45c7-9c0e-b4f9fb4537bd}
        self.add(Invocable::exp("burn", "shell:CD Burning", "")); //TODO: doc
        self.add(Invocable::exp("resource", "shell:ResourceDir", "")); //TODO: doc
        self.add(Invocable::exp("savegame", "shell:SavedGames", "")); //TODO: doc
        self.add(Invocable::exp("sys32", "shell:System", "")); //TODO: doc
        self.add(Invocable::exp("sys86", "shell:Systemx86", "")); //TODO: doc
        self.add(Invocable::exp("roamtile", "shell:Roaming Tiles", "%USERPROFILE%\\AppData\\Local\\Microsoft\\Windows\\RoamingTiles"));
        self.add(Invocable::exp("progf", "shell:ProgramFiles", "C:\\Program Files")); //TODO: doc
        self.add(Invocable::exp("progfc", "shell:ProgramFilesCommon", "C:\\Program Files\\Common Files"));
        self.add(Invocable::exp("progf86", "shell:ProgramFilesX86", "C:\\Program Files (x86)"));
        self.add(Invocable::exp("progfc86", "shell:ProgramFilesCommonX86", "C:\\Program Files (x86)\\Common Files"));
        self.add(Invocable::exp("public", "shell:Public", "")); //TODO: doc
        self.add(Invocable::exp("printhood", "shell:PrintHood", "")); //TODO: doc
        self.add(Invocable::exp("printrsf", "shell:PrintersFolder", "")); //TODO: doc
        self.add(Invocable::exp("programf", "shell:Programs", "")); //TODO: doc
        self.add(Invocable::exp("profilef", "shell:Profile", "")); //TODO: doc
        self.add(Invocable::exp("userpins", "shell:User Pinned", "")); //TODO: doc
        self.add(Invocable::exp("playlist", "shell:playlists", "")); //TODO: doc
        self.add(Invocable::exp("programsc", "shell:Common Programs", "Shared programs folder")); //TODO: doc
        self.add(Invocable::exp("templates", "shell:templates", "")); //TODO: doc
        self.add(Invocable::exp("sendto", "shell:sendto", "")); //TODO: doc
        self.add(Invocable::exp("freq", "shell:::{3936E9E4-D92C-4EEE-A85A-BC16D5EA0819}", "Frequently accessed folders"));
        self.add(Invocable::exp("download", "shell:Downloads", "")); //TODO: doc
        self.add(Invocable::exp("pubdown", "shell:CommonDownloads", "")); //TODO: doc
        self.add(Invocable::exp("pubdesk", "shell:Common Desktop", "Common desktop folder")); //TODO: doc
        self.add(Invocable::exp("desktop", "shell:Desktop", "User desktop folder")); // shell:::{00021400-0000-0000-C000-000000000046}", // error?
        self.add(Invocable::exp("apps", "shell:AppsFolder", "Applications folder")); //{4234d49b-0245-4df3-b780-3893943456e1}
        self.add(Invocable::exp("recyc", "shell:RecycleBinFolder", "Recycle bin folder")); // shell:::{645FF040-5081-101B-9F08-00AA002F954E}
        self.add(Invocable::exp("dpapikey", "shell:DpAPIKeys", "")); //TODO: doc
        self.add(Invocable::exp("pubdocs", "shell:Common Documents", "")); //TODO: doc
        self.add(Invocable::exp("favs", "shell:Favorites", "Favorites folder")); //TODO: doc // shell:::{323CA680-C24D-4099-B94D-446DD2D7249E}
        self.add(Invocable::exp("fontsdir", "shell:Fonts", "")); //TODO: doc
        self.add(Invocable::exp("doclib", "shell:DocumentsLibrary", "")); //TODO: doc
        self.add(Invocable::exp("acctpict", "shell:AccountPictures", "")); //TODO: doc
        self.add(Invocable::exp("pictlib", "shell:PicturesLibrary", "")); //TODO: doc
        self.add(Invocable::exp("links", "shell:Links", "")); //TODO: doc
        self.add(Invocable::exp("quick", "shell:Quick Launch", "")); //TODO: doc
        self.add(Invocable::exp("recent", "shell:recent", "")); //TODO: doc
        self.add(Invocable::exp("addnprog", "shell:AddNewProgramsFolder", "")); //TODO: doc
        self.add(Invocable::exp("chngprog", "shell:ChangeRemoveProgramsFolder", "")); //TODO: doc
        self.add(Invocable::exp("gamexp", "shell:PublicGameTasks", "")); //TODO: doc
        self.add(Invocable::exp("contacts", "shell:Contacts", "")); //TODO: doc
        self.add(Invocable::exp("cookies", "shell:Cookies", "")); //TODO: doc
        self.add(Invocable::exp("creds", "shell:CredentialManager", "")); //TODO: doc
        self.add(Invocable::exp("vidlib", "shell:VideosLibrary", "")); //TODO: doc
        self.add(Invocable::exp("libs", "shell:Libraries", "")); //TODO: doc  //shell:::{031E4825-7B94-4dc3-B131-E946B44C8DD5}
        self.add(Invocable::exp("history", "shell:History", "")); //TODO: doc
        self.add(Invocable::exp("impappsc", "shell:ImplicitAppShortcuts", "")); //TODO: doc
        self.add(Invocable::exp("crypkeys", "shell:Cryptokeys", "")); //TODO: doc
        self.add(Invocable::exp("inetcach", "shell:cache", "")); //TODO: doc
        self.add(Invocable::exp("startme", "shell:Start Menu", "")); //TODO: doc
        self.add(Invocable::exp("startall", "shell:Common Start Menu", "")); //TODO: doc
        self.add(Invocable::exp("windowsf", "shell:Windows", "")); //TODO: doc
        self.add(Invocable::exp("pubvideo", "shell:CommonVideo", "")); //TODO: doc
        self.add(Invocable::exp("myvideo", "shell:My Video", "")); //TODO: doc
        self.add(Invocable::exp("mydocs", "shell:::{450D8FBA-AD25-11D0-98A8-0800361B1103}", "My Documents"));
        self.add(Invocable::exp("mail", "outlookmail:", ""));
        self.add(Invocable::exp("mediasrv", "shell:::{289AF617-1CC3-42A6-926C-E6A863F0E3BA}", "Media Servers"));
        self.add(Invocable::exp("msvideo", "microsoftvideo:", ""));
    }

    /// Add SysInternals utilities Invocables to the list of Invocables in this InvocableCategory.
    /// <https://live.sysinternals.com/> <https://docs.microsoft.com/en-us/sysinternals/>
    pub fn add_sysinternals(&mut self) {
        self.add(Invocable::cmd_with("bginfo", "$syslivebginfo64.exe", "Set desktop background to system information", &["-accepteula"]));
        self.add(Invocable::cmd_with("handle", "$syslivehandle64.exe", "List open file handles", &["-accepteula"]));
        self.add(Invocable::cmd_with("listdlls", "$syslivelistdlls64.exe", "List processes and their DLLs", &["-accepteula"]));
        //TODO: "whoson" => Invoker::cmd(format!("{}whoson64.exe", &["-accepteula"]), // nosuch?
        self.add(Invocable::cmd_with("procexp", "$sysliveprocexp64.exe", "Process Explorer", &["-accepteula"]));
        self.add(Invocable::cmd_with("pslist", "$syslivepslist64.exe", "Process lister", &["-accepteula"]));
        self.add(Invocable::cmd_with("pskill", "$syslivepskill.exe", "Process killer", &["-accepteula"]));
        self.add(Invocable::cmd_with("procmon", "$sysliveprocmon64.exe", "Process Monitor", &["-accepteula"]));
        self.add(Invocable::cmd_with("autoruns", "$sysliveautoruns64.exe", "Identify and control startup processes", &["-accepteula"]));
        self.add(Invocable::cmd_with("diskview", "$syslivediskview64.exe", "Disk space usage visualizer", &["-accepteula"]));
        self.add(Invocable::cmd_with("du", "$syslivedu64.exe", "Disk usage", &["-accepteula"]));
        self.add(Invocable::cmd_with("zoomit", "$syslivezoomit63.exe", "crash it, change it, mail – upgrade it, Charge it, point it, zoom it, press it, Snap it, work it, quick – erase it...Technologic", &["-accepteula"]));
    }

    /// Add Windows settings Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_settings(&mut self) {
        self.add(Invocable::cmd("utilman", "utilman.exe", "Utility Manager (display)"));
        self.add(Invocable::cmd("intl", "intl.cpl", "Region"));
        self.add(Invocable::cmd("fontview", "fontview.exe", "Font Viewer"));
        self.add(Invocable::bin("sigverif", "sigverif.exe", "File Signature Verification"));
        self.add(Invocable::cmd("iscsicpl", "iscsicpl.exe", "iSCSI"));
        self.add(Invocable::exp("srchsets", "ms-settings:cortana-windowssearch", "Windows Search (Cortana) Settings"));
        self.add(Invocable::exp("filehist", "shell:::{F6B6E965-E9B2-444B-9286-10C9152EDBC5}", "File History"));
        self.add(Invocable::exp("syncset", "ms-settings:sync", "Sync Settings...settings"));
        self.add(Invocable::exp("devices", "shell:::{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "Devices and Printers"));
        self.add(Invocable::exp("storsp", "shell:::{F942C606-0914-47AB-BE56-1321B8035096}", "Manage Storage Spaces"));
        self.add(Invocable::exp("control", "shell:ControlPanelFolder", "Control Panel (small icons)")); // shell:::{21EC2020-3AEA-1069-A2DD-08002B30309D}
        self.add(Invocable::exp("cpcats", "shell:::{26EE0668-A00A-44D7-9371-BEB064C98683}", "Control Panel (categories)"));
        self.add(Invocable::exp("fileopt", "shell:::{6DFD7C5C-2451-11d3-A299-00C04F8EF6AF}", "File Explorer settings"));
        self.add(Invocable::cmd_with("oldfonts", "control.exe", "Legacy fonts control panel", &["fonts"])); //TODO: duplicate?
        self.add(Invocable::cmd("perfopt", "SystemPropertiesPerformance.exe", "Performance Options"));
        self.add(Invocable::cmd("devman", "hdwwiz.cpl", "Device Manager control pane")); // devmgmt.msc
        self.add(Invocable::cmd("timedate", "timedate.cpl", "Date and Time control panel"));
        self.add(Invocable::cmd("inet", "inetcpl.cpl", "Internet control panel"));
        self.add(Invocable::cmd("joy", "joy.cpl", "Game Controllers control panel"));
        self.add(Invocable::cmd("loc", "telephon.cpl", "Location Information control panel"));
        self.add(Invocable::cmd("mouse", "main.cpl", "Mouse control panel"));
        self.add(Invocable::exp("power", "shell:::{025A5937-A6BE-4686-A844-36FE4BEC8B6D}", "Power control panel")); // powercfg.cpl
        self.add(Invocable::cmd("sound", "mmsys.cpl", "Sound control panel"));
        self.add(Invocable::cmd("sysprop", "sysdm.cpl", "Sound Properties control panel")); // SystemPropertiesHardware.exe SystemPropertiesAdvanced.exe
        self.add(Invocable::exp("about", "ms-settings:about", "About settings")); // shell:::{BB06C0E4-D293-4f75-8A90-CB05B6477EEE}
        self.add(Invocable::exp("autoplay", "ms-settings:autoplay", "Autoplay Settings (defaults)"));
        self.add(Invocable::exp("autoplyd", "shell:::{9C60DE1E-E5FC-40f4-A487-460851A8D915}", "Autoplay Settings by Device type"));
        self.add(Invocable::exp("battery", "ms-settings:batterysaver", "Battery settings"));
        self.add(Invocable::exp("captures", "ms-settings:gaming-gamedvr", "Screen capture settings"));
        self.add(Invocable::exp("clp", "ms-settings:clipboard", "Clipboard settings"));
        self.add(Invocable::exp("condev", "ms-settings:connecteddevices", "Connected devices (bluetooth and other devices) settings"));
        self.add(Invocable::exp("crossdev", "ms-settings:crossdevice", "Shared experiences accross devices settings"));
        self.add(Invocable::exp("datause", "ms-settings:datausage", "Data usage settings"));
        self.add(Invocable::exp("default", "ms-settings:defaultapps", "Choose default applications settings")); // shell:::{2559a1f7-21d7-11d4-bdaf-00c04f60b9f0} // computerdefaults
        self.add(Invocable::exp("devdisc", "ms-settings-connectabledevices:devicediscovery", "Connectable device discovery settings"));
        self.add(Invocable::exp("focus", "ms-settings:quiethours", "Quiet hours/focus assist settungs")); // ms-settings:quietmomentshome ms-settings:quietmomentsscheduled ms-settings:quietmomentspresentation ms-settings:quietmomentsgame
        self.add(Invocable::exp("morfonts", "shell:::{93412589-74D4-4E4E-AD0E-E0CB621440FD}", "More Font settings"));
        self.add(Invocable::exp("gamemode", "ms-settings:gaming-gamemode", "Game mode settings"));
        self.add(Invocable::exp("graphics", "ms-settings:display-advancedgraphics", "Advanced graphics settings"));
        self.add(Invocable::exp("keyboard", "ms-settings:keyboard", "Keyboard settings")); //TODO: language?
        self.add(Invocable::exp("multitsk", "ms-settings:multitasking", "Multitasking settings"));
        self.add(Invocable::exp("nags", "ms-settings:notifications", "Notification settings"));
        self.add(Invocable::exp("night", "ms-settings:nightlight", "Night light settings"));
        self.add(Invocable::exp("maps", "ms-settings:maps", "Map settings"));
        self.add(Invocable::exp("offlmaps", "ms-settings:maps-downloadmaps", "Offline map settings"));
        self.add(Invocable::exp("pen", "ms-settings:pen", "Pen input settings"));
        self.add(Invocable::exp("rdset", "ms-settings:remotedesktop", "")); //TODO: doc
        self.add(Invocable::exp("scaling", "ms-settings:display-advanced", "")); //TODO: doc
        self.add(Invocable::exp("prntscan", "ms-settings:printers", "Printer and Scanner settings"));
        self.add(Invocable::exp("setemail", "ms-settings:emailandaccounts", "Email and Accounts settings"));
        self.add(Invocable::exp("setspch", "ms-settings:speech", "")); //TODO: doc
        self.add(Invocable::exp("speech", "shell:::{58E3C745-D971-4081-9034-86E34B30836A}", "Configure your speech recognition experience")); //TODO: doc
        self.add(Invocable::exp("setstart", "ms-settings:startupapps", "")); //TODO: doc
        self.add(Invocable::exp("setvideo", "ms-settings:videoplayback", "")); //TODO: doc
        self.add(Invocable::exp("sounddev", "ms-settings:sound-devices", "")); //TODO: doc
        self.add(Invocable::exp("sounds", "ms-settings:sound", "Sound settings")); //TODO: doc
        self.add(Invocable::exp("storpol", "ms-settings:storagepolicies", "Storage Policies")); //TODO: doc
        self.add(Invocable::exp("storsens", "ms-settings:storagesense", "Storage Sense"));
        self.add(Invocable::exp("tablet", "ms-settings:tabletmode", "Tablet Mode settings"));
        self.add(Invocable::exp("themes", "ms-settings:themes", "Windows Themes settings"));
        self.add(Invocable::exp("touchpad", "ms-settings:devices-touchpad", "Touchpad settings"));
        self.add(Invocable::exp("typing", "ms-settings:typing", "Typing settings"));
        self.add(Invocable::exp("usb", "ms-settings:usb", "USB settings"));
        self.add(Invocable::exp("webapps", "ms-settings:appsforwebsites", "")); //TODO: doc
        self.add(Invocable::exp("workplc", "ms-settings:workplace", "Workplace or school settings"));
        self.add(Invocable::exp("yourinfo", "ms-settings:yourinfo", "Your Information"));
        self.add(Invocable::exp("pows", "ms-settings:powersleep", "Power and Sleep settings"));
        self.add(Invocable::exp("project", "ms-settings-displays-topology:projection", "Project (dark gray sidebar at right of screen)"));
        self.add(Invocable::exp("projectme", "ms-settings:project", "Projecting to this PC"));
        self.add(Invocable::exp("background", "ms-settings:personalization-background", "Desktop background settings"));
        self.add(Invocable::exp("colors", "ms-settings:personalization-colors", "Windows Colors settings")); // ms-settings:colors
        self.add(Invocable::exp("datetime", "ms-settings:dateandtime", "Date and time settings"));
        self.add(Invocable::exp("dev", "ms-settings:developers", "Developer settings"));
        self.add(Invocable::exp("display", "ms-settings:display", "Display settings")); // ms-settings:screenrotation // ms-settings:easeofaccess-colorfilter-adaptivecolorlink // ms-settings:easeofaccess-colorfilter-bluelightlink // desk.cpl
        self.add(Invocable::exp("gamebar", "ms-settings:gaming-gamebar", "Game bar settings"));
        self.add(Invocable::exp("lock", "ms-settings:lockscreen", "Lock screen settings"));
        self.add(Invocable::exp("mouseset", "ms-settings:mousetouchpad", "Mouse settings"));
        self.add(Invocable::exp("personal", "shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921}", "Personalization settings")); //TODO: ms-settings:personalization is desktop background?
        self.add(Invocable::exp("taskbar", "ms-settings:taskbar", "Taskbar settings")); // shell:::{0DF44EAA-FF21-4412-828E-260A8728E7F1}
        self.add(Invocable::exp("vol", "ms-settings:apps-volume", "Volume settings"));
        self.add(Invocable::exp("activatn", "ms-settings:activation", "Windows activation settings"));
        self.add(Invocable::cmd("activate", "slui.exe", "Activate Windows"));
        self.add(Invocable::exp("backup", "ms-settings:backup", "Backup settings"));
        self.add(Invocable::exp("recover", "ms-settings:recovery", "Recovery settings"));
        self.add(Invocable::exp("otherusr", "ms-settings:otherusers", "Other users settings"));
        self.add(Invocable::exp("findmydv", "ms-settings:findmydevice", "Find my device settings"));
        self.add(Invocable::exp("region", "ms-settings:regionformatting", "Regional formatting settings"));
        self.add(Invocable::exp("language", "ms-settings:regionlanguage", "Regional language settings")); // ms-settings:regionlanguage-languageoptions ms-settings:regionlanguage-setdisplaylanguage ms-settings:regionlanguage-adddisplaylanguage
        self.add(Invocable::exp("settings", "ms-settings:", "Settings control panel"));
        self.add(Invocable::exp("start", "ms-settings:personalization-start", "Start Menu personalization settings"));
        self.add(Invocable::exp("startfol", "ms-settings:personalization-start-places", "Personalize/choose which folders appear on the start menu"));
    }

    /// Add Windows programs and features Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_features(&mut self) {
        self.add(Invocable::exp("features", "ms-settings:appsfeatures", "Optional Apps and Features Settings")); // optionalfeatures.exe shell:::{67718415-c450-4f3c-bf8a-b487642dc39b}
        self.add(Invocable::exp("optional", "ms-settings:optionalfeatures", "Optional Features Settings"));
        self.add(Invocable::cmd("programs", "appwiz.cpl", "Uninstall or Change a Program"));
        self.add(Invocable::exp("defaults", "shell:::{17cd9488-1228-4b2f-88ce-4298e93e0966}", "Default Programs"));
        self.add(Invocable::exp("update", "ms-settings:windowsupdate-action", "Windows Update"));
        self.add(Invocable::exp("updateh", "ms-settings:windowsupdate-history", "Windows Update History"));
        self.add(Invocable::exp("updater", "ms-settings:windowsupdate-restartoptions", "Windows Update Restart Options Settings"));
        self.add(Invocable::exp("updateo", "ms-settings:windowsupdate-options", "Windows Update Advanced Option Settingss"));
        self.add(Invocable::exp("updateah", "ms-settings:windowsupdate-activehours", "Windows Update Active Hours Settings"));
        self.add(Invocable::exp("delivopt", "ms-settings:delivery-optimization", "Microsoft Updates Delivery Optimization Settings"));
    }

    /// Add networking components to the list of Invocables in this InvocableCategory.
    pub fn add_networking(&mut self) {
        self.add(Invocable::cmd_with("flushdns", "ipconfig.exe", "Flush DNS Cache", &["/flushdns"]));
        self.add(Invocable::exp("remoteapp", "shell:::{241D7C96-F8BF-4F85-B01F-E2B043341A4B}", "RemoteApp and Desktop Connections"));
        self.add(Invocable::exp("yurphone", "ms-settings:mobile-devices", "Mobile Devices/Your Phone"));
        self.add(Invocable::exp("addphone", "ms-settings:mobile-devices-addphone-direct", "Mobile Devices/Add Phone"));
        self.add(Invocable::exp("addnetp", "shell:::{D4480A50-BA28-11d1-8E75-00C04FA31A86}", "Add Network Place"));
        self.add(Invocable::exp("netcon", "shell:ConnectionsFolder", "Network Connections folder")); // ncpa.cpl
        self.add(Invocable::exp("netshare", "shell:::{8E908FC9-BECC-40f6-915B-F4CA0E70D03D}", "Network and Sharing Center"));
        self.add(Invocable::exp("netavail", "ms-availablenetworks:", "Available networks"));
        self.add(Invocable::exp("nethood", "shell:NetHood", "Network Shortcuts"));
        self.add(Invocable::exp("dialup", "ms-settings:network-dialup", "Dialup Networking settings"));
        self.add(Invocable::exp("ethernet", "ms-settings:network-ethernet", "Ethernet LAN Cable networking settings"));
        self.add(Invocable::exp("hotspot", "ms-settings:network-mobilehotspot", "Mobile Hotspot Wi-Fi network settings"));
        self.add(Invocable::exp("proxy", "ms-settings:network-proxy", "Network Proxy settings"));
        self.add(Invocable::exp("vpn", "ms-settings:network-vpn", "Virtual Private Network settings"));
        self.add(Invocable::exp("wheelsup", "ms-settings:network-airplanemode", "Airplane Mode settingse"));
        self.add(Invocable::exp("wifinets", "ms-settings:network-wifisettings", "Wi-Fi Network settings"));
        self.add(Invocable::exp("wifisets", "ms-settings:network-wifi", "Wi-Fi Network settings"));
        self.add(Invocable::exp("bluetoo", "ms-settings:bluetooth", "Bluetooth settings"));
        self.add(Invocable::exp("netstat", "ms-settings:network", "Network status settings"));
    }

    /// Add Windows privacy controls Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_privacy(&mut self) {
        self.add(Invocable::exp("srchprm", "ms-settings:search-permissions", "Permissions and History"));
        self.add(Invocable::exp("privacy", "ms-settings:privacy", "Privacy settings control panel"));
        self.add(Invocable::exp("pdoc", "ms-settings:privacy-documents", "Documents privacy settings"));
        self.add(Invocable::exp("pfs", "ms-settings:privacy-broadfilesystemaccess", "File system access privacy settings"));
        self.add(Invocable::exp("miccheck", "ms-settings:privacy-microphone", "Microphone privacy settings"));
        self.add(Invocable::exp("pappdiag", "ms-settings:privacy-appdiagnostics", "App Diagnostics privacy settings"));
        self.add(Invocable::exp("pautodwn", "ms-settings:privacy-automaticfiledownloads", "Automatic File Downloads privacy settings"));
        self.add(Invocable::exp("pbackapp", "ms-settings:privacy-backgroundapps", "Background Apps privacy settings"));
        self.add(Invocable::exp("pcalls", "ms-settings:privacy-phonecalls", "Phone Calls privacy settings"));
        self.add(Invocable::exp("pcals", "ms-settings:privacy-calendar", "Calendar privacy settings"));
        self.add(Invocable::exp("pcallh", "ms-settings:privacy-callhistory", "Call History privacy settings"));
        self.add(Invocable::exp("pcam", "ms-settings:privacy-webcam", "")); //TODO: doc
        self.add(Invocable::exp("pcontact", "ms-settings:privacy-contacts", "")); //TODO: doc
        self.add(Invocable::exp("pdev", "ms-settings:privacy-customdevices", "Custom/Other Devices privacy settings")); //TODO: doc
        self.add(Invocable::exp("pdocs", "ms-settings:privacy-documents", "")); //TODO: doc
        self.add(Invocable::exp("pemail", "ms-settings:privacy-email", "Email privacy settings"));
        self.add(Invocable::exp("pfeed", "ms-settings:privacy-feedback", "")); //TODO: doc
        self.add(Invocable::exp("phist", "ms-settings:privacy-activityhistory", "")); //TODO: doc
        self.add(Invocable::exp("pmsg", "ms-settings:privacy-messaging", "Messaging privacy settings"));
        self.add(Invocable::exp("pmyinfo", "ms-settings:privacy-accountinfo", "Accoung Information privacy settings")); //TODO: doc
        self.add(Invocable::exp("pnags", "ms-settings:privacy-notifications", "Notifications privacy settings")); //TODO: doc
        self.add(Invocable::exp("ppics", "ms-settings:privacy-pictures", "")); //TODO: doc
        self.add(Invocable::exp("pradios", "ms-settings:privacy-radios", "")); //TODO: doc
        self.add(Invocable::exp("pspeech", "ms-settings:privacy-speech", "")); //TODO: doc
        self.add(Invocable::exp("ptasks", "ms-settings:privacy-tasks", "")); //TODO: doc
        self.add(Invocable::exp("ptype", "ms-settings:privacy-speechtyping", "")); //TODO: doc
        self.add(Invocable::exp("pvoicea", "ms-settings:privacy-voiceactivation", "")); //TODO: doc
        self.add(Invocable::exp("stalkme", "ms-settings:privacy-location", "Location privacy settings"));
    }

    /// Add shutdown commands to the list of Invocables in this InvocableCategory.
    pub fn add_shutdown(&mut self) {
        self.add(Invocable::bin_with("boot", "shutdown.exe", "Reboot", &["/r", "/t", "10"])); // "/t", "30"]), //reboot in 30 seconds unless shutdown.exe /a
        self.add(Invocable::bin_with("bootopt", "shutdown.exe", "Reboot to boot options", &["/r", "/o"])); //"/t", "30"]), // reboot to boot options in 30 seconds unless shutdown.exe /a
        self.add(Invocable::bin_with("down", "shutdown.exe", "Shut down", &["/s"])); // "/t", "30"]), // shut down in 30 seconds unless shutdown.exe /a
        self.add(Invocable::bin_with("firmware", "shutdown.exe", "Reboot to firmware", &["/r", "/fw"])); // "/t", "30"]), // reboot to formware in 30 seconds unless shutdown.exe /a
        self.add(Invocable::bin_with("hyb", "shutdown.exe", "Hybernate", &["/h"])); // err, "/t", "30"]), // hybernate
        self.add(Invocable::bin_with("logoff", "shutdown.exe", "Log off", &["/l"])); // "/t", "30"]), // logoff in 30 seconds unless shutdown.exe /a
        self.add(Invocable::bin_with("shutui", "shutdown.exe", "Shutdown UI", &["/i"]));
        // reboot to boot options in 30 seconds unless shutdown.exe /a         //TODO: new/doc
    }

    /// Add Windows Ease of Access settings Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_ease_of_access(&mut self) {
        self.add(Invocable::exp("eatcur", "ms-settings:easeofaccess-cursor", "Ease of Access text cursor settings"));
        self.add(Invocable::exp("eamag", "ms-settings:easeofaccess-magnifier", "Ease of Access magnifier"));
        self.add(Invocable::exp("eacolflt", "ms-settings:easeofaccess-colorfilter", "Ease of Access color filter"));
        self.add(Invocable::exp("eahighc", "ms-settings:easeofaccess-highcontrast", "Ease of Access high contrast"));
        self.add(Invocable::exp("eaeyec", "ms-settings:easeofaccess-eyecontrol", "Ease of Access eye control"));
        self.add(Invocable::exp("eamouse", "ms-settings:easeofaccess-mouse", "Ease of Access mouse settings"));
        self.add(Invocable::exp("eakeyb", "ms-settings:easeofaccess-keyboard", "Ease of Access keyboard settings"));
        self.add(Invocable::exp("easpeech", "ms-settings:easeofaccess-speechrecognition", "Ease of Access speech recognition settings"));
        self.add(Invocable::exp("eaaudio", "ms-settings:easeofaccess-audio", "Ease of Access audio settings"));
        self.add(Invocable::exp("eadisp", "ms-settings:easeofaccess-display", "Ease of Access display settings"));
        self.add(Invocable::exp("eanar", "ms-settings:easeofaccess-narrator", "Ease of Access narrator settings")); // ms-settings:easeofaccess-narrator-isautostartenabled
        self.add(Invocable::exp("ease", "shell:::{D555645E-D4F8-4c29-A827-D93C859C4F2A}", "Ease of Access Settings")); // "control.exe",  &["access.cpl"]));
        self.add(Invocable::exp("captions", "ms-settings:easeofaccess-closedcaptioning", "Ease of Access closed captioning settings"));
    }

    /// Add security-related Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_security(&mut self) {
        self.add(Invocable::bin("rekeywiz", "rekeywiz.exe", "Encrypt Filesystem with Certificate"));
        self.add(Invocable::cmd("credwiz", "credwiz.exe", "Stored Usernames and Passwords"));
        self.add(Invocable::cmd("secpol", "secpol.msc", "Security Policy"));
        self.add(Invocable::cmd("gpedit", "gpedit.msc", "Local Group Policy Editor"));
        self.add(Invocable::cmd("lusrmgr", "lusrmgr.msc", "Local Users and Groups"));
        self.add(Invocable::exp("user", "shell:::{60632754-c523-4b62-b45c-4172da012619}", "User Accounts"));
        self.add(Invocable::cmd_with("userpass", "control.exe", "", &["userpasswords2"]));
        self.add(Invocable::cmd("psr", "psr.exe", "Steps Recorder")); //TODO: was "Password Safe Repository"));
        self.add(Invocable::exp("syscerts", "shell:SystemCertificates", "System Security Certificates"));
        self.add(Invocable::cmd_with("eup", "rundll32.exe", "Edit User Profiles", &["sysdm.cpl,EditUserProfiles"]));
        self.add(Invocable::cmd("secman", "wscui.cpl", "Security and Maintenance control panel"));
        self.add(Invocable::cmd("firewall", "firewall.cpl", "Firewall security settings"));
        self.add(Invocable::cmd("advsec", "wf.msc", "Advanced firewall security settings"));
        self.add(Invocable::exp("facesign", "ms-settings:signinoptions-launchfaceenrollment", "Face recognition security settings"));
        self.add(Invocable::exp("fingsign", "ms-settings:signinoptions-launchfingerprintenrollment", "Fingerprint recognition security settings"));
        self.add(Invocable::exp("keysign", "ms-settings:signinoptions-launchsecuritykeyenrollment", "Security key (USB) security settings"));
        self.add(Invocable::exp("upsign", "ms-settings:signinoptions-dynamiclock", "Security dynamic lock settings"));
        self.add(Invocable::exp("signin", "ms-settings:signinoptions", "Security sign-in settings"));
        self.add(Invocable::exp("seccntr", "windowsdefender:", "Windows Security Center/Security at a Glance"));
        self.add(Invocable::exp("winsec", "ms-settings:windowsdefender", "Windows Security Settings"));
        self.add(Invocable::exp("bitlock", "shell:::{D9EF8727-CAC2-4e60-809E-86F80A666C91}", "Bitlocker Drive Encryption"));
        self.add(Invocable::cmd("authman", "azman.msc", "Security Authorization Manager"));
        self.add(Invocable::cmd("certmgr", "certmgr.msc", "Security Certificate Manager - Current User"));
        self.add(Invocable::cmd("certlm", "certlm.msc", "Security Certificate Manager - Local Machine"));
        self.add(Invocable::cmd("useracts", "netplwiz.exe", "Security User Accounts"));
        self.add(Invocable::exp("users", "shell:userprofiles", "User Profiles"));
        self.add(Invocable::cmd("uac", "UserAccountControlSettings.exe", ""));
    }

    /// Add Linux Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_linux(&mut self) {
        self.add(Invocable::sh("wince", "/home/jw/bin/wince", "Run the shell script that recompiles this program"));
        self.add(Invocable::sh("gowindow", "mkdir /mnt/temp/GoWindow.{ED7BA470-8E54-465E-825C-99712043E01C} 2>/dev/null ; explorer.exe 'C:\\temp\\GoWindow.{ED7BA470-8E54-465E-825C-99712043E01C}'", "GoWindow (God Mode)"));
        /*
                self.add(Invocable::sh(
                    "ntt",
                    "
                    if [ \"$1\" = \"\" ]; then
                        cmd.exe /c wt.exe -w 0 nt bash.exe -c \"$0 $PWD\" 2>/dev/null
                    else
                        cd $1
                        bash.exe
                    fi",
                    "New Windows Terminal Tab in current directory",
                ));
        */
        self.add(Invocable::sh("bash", "", "Run the Unix command specified on the command line"));
        self.add(Invocable::sh("gimp", "/usr/bin/gimp", "gimp (image manipulation)"));
        self.add(Invocable::sh("microsoft-edge", "/usr/bin/microsoft-edge-dev", "microsoft-edge (brower)"));
        self.add(Invocable::sh("google-chrome", "/usr/bin/google-chrome", "google-chrome (browser)"));
        self.add(Invocable::sh("gedit", "/usr/bin/gedit", "gedit (graphical editor)"));
        self.add(Invocable::sh("xlogo", "/usr/bin/xlogo", "xlogo (visual X logo)"));
        self.add(Invocable::sh("xmore", "/usr/bin/xmore", "xmore (read-only text UI)"));
        self.add(Invocable::sh("xgc", "/usr/bin/xgc", "xgc (graphics demo)"));
        self.add(Invocable::sh("xman", "/usr/bin/xman", "xman (man pages)"));
        self.add(Invocable::sh("xcalc", "/usr/bin/xcalc", "xcalc (calculator)"));
        self.add(Invocable::sh("xeyes", "/usr/bin/xeyes", "xeyes (visual eyeballs)"));
        self.add(Invocable::sh("xclock", "/usr/bin/xclock", "xclock (visual clock)"));
        self.add(Invocable::sh("lvlc", "/usr/bin/lvlc", "Linux VLC (media player)"));
        self.add(Invocable::sh("nautilus", "/usr/bin/nautilus", "nautilus (file browser)"));
    }

    /// Add miscelaneous Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_miscellaneous(&mut self) {
        self.add(Invocable::exp("insider", "ms-settings:windowsinsider", "Microsoft Windows Insider Program"));
        self.add(Invocable::exp("wintab", "shell:::{3080F90E-D7AD-11D9-BD98-0000947B0257}", "Switch windows (Windows+Tab)"));
        self.add(Invocable::exp("showd", "shell:::{3080F90D-D7AD-11D9-BD98-0000947B0257}", "Show Windows desktop"));
        self.add(Invocable::exp("trouble", "ms-settings:troubleshoot", "Troubleshooting Windows"));
        self.add(Invocable::cmd("quickass", "quickassist.exe", "Windows Quick Assist"));
        //        self.add(Invocable::cmd_with("wupdate", "runas.exe", "Update WSL", &["/env", "/user:administrator", "wsl.exe --update"])); // runas from wink does not allow password entry
    }

    /// Add Windows utilities Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_utilities(&mut self) {
        self.add(Invocable::bin("iexpress", "iexpress.exe", "Create a Self-Extracting Executable"));
        self.add(Invocable::bin("cleanmgr", "cleanmgr.exe", "Disk Cleanup"));
        self.add(Invocable::cmd("shrpubw", "shrpubw.exe", "Create Shared Folder Wiard"));
        self.add(Invocable::cmd("rasphone", "rasphone.exe", "Remove Access Phonebook"));
        self.add(Invocable::bin("fxscover", "fxscover.exe", "Fax Cover Page Editor"));
        self.add(Invocable::bin("tabcal", "tabcal.exe", "Digitizer Calibration Tool"));
        self.add(Invocable::cmd("cliconfg", "cliconfg.exe", "SQL Server Client Network Utility"));
        self.add(Invocable::cmd("dpapimig", "dpapimig.exe", "Protected Content Migration"));
        self.add(Invocable::cmd("printmig", "printbrmui.exe", "Printer Migration"));
        self.add(Invocable::cmd("presset", "PresentationSettings.exe", "Presentation Settings"));
        self.add(Invocable::exp("kiosk", "ms-settings:assignedaccess", "Set up a Kiosk"));
        self.add(Invocable::cmd("diskman", "diskmgmt.msc", "Disk Management"));
        self.add(Invocable::cmd("mmc", "mmc.exe", "Microsoft Management Console"));
        self.add(Invocable::cmd("rsop", "rsop.mmc", "Resultant Set of Policy"));
        self.add(Invocable::cmd("printman", "printmanagement.msc", "Print Management"));
        self.add(Invocable::exp("run", "shell:::{2559a1f3-21d7-11d4-bdaf-00c04f60b9f0}", "Windows Run Dialog"));
        self.add(Invocable::cmd("lpksetup", "lpksetup.exe", "Language Pack Setup"));
        self.add(Invocable::cmd("msinfo32", "msinfo32.exe", "System Information"));
        self.add(Invocable::cmd("verifier", "verifier.exe", "Driver Verifier Manager"));
        self.add(Invocable::cmd("iexplore", "$pf64/Internet Explorer/iexplore.exe", "Internet Explorer"));
        self.add(Invocable::cmd_with("pwrd", "rundll32.exe", "keymgr.dll,PRShowSaveWizardExW", &["keymgr.dll,PRShowSaveWizardExW"])); //TODO: doc
        self.add(Invocable::cmd("tpm", "tpminit.exe", "Trusted Platform Module"));
        self.add(Invocable::cmd("tpm.msc", "tpm.msc", "Trusted Platform Module Management on Local Computer"));
        self.add(Invocable::cmd("devmode", "DevModeRunAsUserConfig.msc", "")); //TODO: doc
        self.add(Invocable::cmd("odbcconf", "odbcconf.exe", ""));
        self.add(Invocable::cmd("wmimgmt", "wmimgmt.msc", "WMI Management"));
        self.add(Invocable::cmd("fsquirt", "fsquirt.exe", "Bluetooth File Transfer"));
        self.add(Invocable::cmd_with("wintools", "control.exe", "Windows Tools", &["admintools"])); //shell:::{D20EA4E1-3957-11d2-A40B-0C5020524153}
        self.add(Invocable::bin("charmap", "charmap.exe", "Character Map"));
        self.add(Invocable::cmd("cttune", "cttune.exe", "Clear Type Text Tuner"));
        self.add(Invocable::cmd("colorcpl", "colorcpl", "Color Management"));
        self.add(Invocable::cmd("compmgmt", "compmgmt.msc", "Computer Management"));
        self.add(Invocable::cmd_with("editenv", "rundll32.exe", "Edit environment variables", &["sysdm.cpl,EditEnvironmentVariables"]));
        self.add(Invocable::cmd("comserv", "dcomcnfg.exe", "Component Services Configuration")); // comexp.msc
        self.add(Invocable::cmd("printui", "printui.exe", "Print User Interface"));
        self.add(Invocable::cmd("eudcedit", "eudcedit.exe", "Private Character Editor"));
        self.add(Invocable::cmd("osk", "osk.exe", "On-screen keyboard"));
        self.add(Invocable::bin("psise", "powershell_ise.exe", "PowerShell Integrated Scripting Environment (ISE)"));
        self.add(Invocable::bin("winver", "winver.exe", "Windows Version"));
        self.add(Invocable::bin("cdinfo", "$pf64/CrystalDiskInfo/DiskInfo64.exe", "Crystal Disk Info"));
        self.add(Invocable::exp("mobility", "shell:::{5ea4f148-308c-46d7-98a9-49041b1dd468}", "Windows Mobility Center")); // mblctr.exe
        self.add(Invocable::cmd("backup7", "sdclt.exe", "Windows 7 backup"));
        self.add(Invocable::cmd("chkdsk", "chkdsk.exe", "Check hard disk for errors and issues"));
        self.add(Invocable::cmd("cmd", "", "cmd.exe (see above)"));
        self.add(Invocable::exp("exp", "", "explorer.exe (see above)"));
        self.add(Invocable::cmd("env", "set", "Show Windows environment"));
        self.add(Invocable::exp("support", "ms-contact-support:", "Microsoft support"));
        self.add(Invocable::exp("movies", "mswindowsvideo:", "Microsoft Windows Video (Movies and TV)"));
        self.add(Invocable::exp("groove", "mswindowsmusic:", "Microsoft Groove Music"));
        self.add(Invocable::exp("bingmaps", "bingmaps:", "Bing Maps"));
        self.add(Invocable::exp("bingnews", "bingnews:", "Bing News"));
        self.add(Invocable::exp("msact", "ms-actioncenter:", "Windows Action Center (right dark gray sidebar)"));
        self.add(Invocable::exp("fam", "ms-wpc:", "Ask for permission (family)"));
        self.add(Invocable::bin("isoburn", "isoburn.exe", "ISO disk burner"));
        self.add(Invocable::bin("notepad", "notepad.exe", "Windows Notepad"));
        self.add(Invocable::bin("wordpad", "$pf86/Windows NT/Accessories/wordpad.exe", "Windows Wordpad"));
        self.add(Invocable::bin("mp", "pf64/windows media player/wmplayer.exe", "Windows Media Player"));
        self.add(Invocable::exp("sync", "shell:::{9C73F5E5-7AE7-4E32-A8E8-8D23B85255BF}", "Sync Center")); // mobsync
        self.add(Invocable::bin("paint", "paint.exe", "Windows Paint"));
        self.add(Invocable::bin("faxscan", "wfs.exe", "Windows Fax and Scan"));
        self.add(Invocable::bin("scan", "wiaacmgr.exe", "Scan"));
        self.add(Invocable::bin("rd", "mstsc.exe", "Remote Desktop Client"));
        self.add(Invocable::bin("msdt", "msdt.exe", "Microsoft Support Diagnostics Tool"));
        self.add(Invocable::bin("dxdiag", "dxdiag.exe", "DirectX Diagnostics Tool"));
        self.add(Invocable::bin("dfrgui", "dfrgui.exe", "Optimize Hard Drives"));
        self.add(Invocable::exp("soundrec", "shell:appsFolder\\Microsoft.WindowsSoundRecorder_8wekyb3d8bbwe!App", "Sound Recorder")); // TODO: wrong
        self.add(Invocable::exp("stikynot", "shell:appsFolder\\Microsoft.MicrosoftStickyNotes_8wekyb3d8bbwe!App", "Sticky Notes")); // TODO: wrong
        self.add(Invocable::exp("alarms", "shell:AppsFolder\\Microsoft.WindowsAlarms_8wekyb3d8bbwe!App", "Alarmns & Clock")); // TODO: wrong
        self.add(Invocable::exp("calc", "calculator:", "Calculator"));
        self.add(Invocable::exp("clock", "ms-clock:", "Clock"));
        self.add(Invocable::exp("cam", "microsoft.windows.camera:", "Camera"));
        self.add(Invocable::exp("cal", "outlookcal:", "Calendar"));
        self.add(Invocable::exp("paint3d", "ms-paint:", "Paint3D"));
        self.add(Invocable::exp("people", "ms-people:", "People"));
        self.add(Invocable::exp("photos", "ms-photos:", "Photos and Video Editor"));
        self.add(Invocable::exp("sclip", "ms-screenclip:", "Screen capture (Windows+Shift+S)"));
        self.add(Invocable::exp("ssketch", "ms-ScreenSketch:", "Snip and sketch"));
        self.add(Invocable::cmd("sniptool", "SnippingTool.exe", "SnippingTool"));
        self.add(Invocable::exp("store", "ms-windows-store:", "Microsoft store"));
        self.add(Invocable::exp("tips", "ms-get-started:", "Windows tips / getting started"));
        self.add(Invocable::exp("sol", "xboxliveapp-1297287741:", "Solitare"));
        self.add(Invocable::cmd("remoteas", "msra.exe", "Windows Remote Assistance"));
        self.add(Invocable::cmd("wusa", "wusa.exe", "Windows Update Standalone Installer"));
        self.add(Invocable::cmd("perfmon", "perfmon.msc", "Performance Monitor"));
        self.add(Invocable::cmd("hdwwiz", "hdwwiz.exe", "Add Hardware Wizard"));
        self.add(Invocable::cmd("dialer", "dialer.exe", "Phone Dialer"));
        self.add(Invocable::cmd("diskpart", "diskpart.exe", "Disk partitioner"));
        self.add(Invocable::cmd("magnify", "magnify.exe", "Screen magnifier"));
        self.add(Invocable::cmd("mdsched", "mdsched.exe", "Windows Memory Diagnostics"));
        self.add(Invocable::cmd("msconfig", "msconfig.exe", "Microsoft Configuration tool"));
        self.add(Invocable::cmd("recdisc", "recdisc.exe", "Create a system repair disk"));
        self.add(Invocable::cmd("restore", "rstrui.exe", "Restore system files and settings"));
        self.add(Invocable::cmd("sndvol", "sndvol.exe", "Sound and Volume"));
        self.add(Invocable::cmd("taskmgr", "taskmgr.exe", "Windows Task Manager"));
        self.add(Invocable::cmd("taskschd", "taskschd.msc", "Windows Task Scheduler"));
        self.add(Invocable::bin("dvdplay", "dvdplay.exe", "DVD player (Windows Media Player)"));
        self.add(Invocable::cmd("eventvwr", "eventvwr.msc", "Windows Event Viewer"));
        self.add(Invocable::cmd("regedt32", "regedt32.exe", "Windows Registry Editor"));
        self.add(Invocable::cmd("resmon", "resmon.exe", "Windows Resource Monitor"));
        self.add(Invocable::cmd("services", "services.msc", "Windows Services"));
        self.add(Invocable::cmd("mrt", "mrt.exe", "Malicious Software Removal Tool"));
    }

    /// Add known screen savers to the list of Invocables in this InvocableCategory.
    pub fn add_screensavers(&mut self) {
        self.add(Invocable::bin_with("sss", "control.exe", "Screen Saver Settings", &["desk.cpl,,@screensaver"]));
        self.add(Invocable::bin_with("defaultss", "powershell.exe", "Default Screen Saver", &["-command", "&(Get-ItemProperty 'HKCU:Control Panel\\Desktop').{SCRNSAVE.EXE}"]));
        self.add(Invocable::bin_with("blank", "scrnsave.scr", "Blank Screen Saver", &["/s"])); // no settings (/c), no window support (/t)
        self.add(Invocable::bin_with("bubbles", "Bubbles.scr", "Bubbles Screen Saver", &["/s"]));
        self.add(Invocable::bin_with("bubbless", "Bubbles.scr", "Bubbles Screen Saver settings", &["/c"]));
        self.add(Invocable::bin_with("bubblesw", "Bubbles.scr", "Bubbles Screen Saver window", &["/t"]));
        self.add(Invocable::bin_with("mystify", "Mystify.scr", "Mystify Screen Saver", &["/s"])); // no settings
        self.add(Invocable::bin_with("mystifyw", "Mystify.scr", "Mystify Screen Saver window", &["/t"]));
        self.add(Invocable::bin_with("photoss", "PhotoScreensaver.scr", "Photos Screen Saver", &["/s"]));
        self.add(Invocable::bin_with("photosss", "PhotoScreensaver.scr", "Photos Screen Saver settings", &["/c"]));
        self.add(Invocable::bin_with("photossw", "PhotoScreensaver.scr", "Photos Screen Saver window", &["/t"]));
        self.add(Invocable::bin_with("ribbons", "ribbons.scr", "Ribbons Screen Saver", &["/s"]));
        self.add(Invocable::bin_with("ribbonss", "ribbons.scr", "Ribbons Screen Saver settings", &["/c"]));
        self.add(Invocable::bin_with("ribbonsw", "ribbons.scr", "Ribbons Screen Saver window", &["/t"]));
        self.add(Invocable::bin_with("3dtss", "ssText3d.scr", "3D Text Screen Saver", &["/s"]));
        self.add(Invocable::bin_with("3dtsss", "ssText3d.scr", "3D Text Screen Saver settings", &["/c"]));
        self.add(Invocable::bin_with("3dtssw", "ssText3d.scr", "3D Text Screen Saver window", &["/t"]));
    }

    /// Add various Windows application Invocables to the list of Invocables in this InvocableCategory.
    pub fn add_applications(&mut self) {
        self.add(Invocable::bin("skype", "$pf86/Microsoft/Skype for Desktop/Skype.exe", "Skype"));
        self.add(Invocable::bin("spotify", "$userpath/AppData/Roaming/Spotify/Spotify.exe", "Spotify"));
        self.add(Invocable::bin("mdp", "$pf86/MarkdownPad 2/MarkdownPad2.exe", "MarkdownPad2"));
        self.add(Invocable::bin("postman", "$userpath/AppData/Local/Postman/Postman.exe", "Postman"));
        self.add(Invocable::bin("zoom", "$userpath/AppData/Roaming/Zoom/bin/Zoom.exe", "Zoom"));
        self.add(Invocable::cmd_with("killzoom", "taskkill.exe", "Kill Zoom", &["/t", "/f", "/im", "zoom.exe"]));
        self.add(Invocable::cmd_with("killslak", "taskkill.exe", "Kill Zoom", &["/t", "/f", "/im", "slack.exe"]));
        self.add(Invocable::bin("ransack", "$pf64/Mythicsoft/Agent Ransack/AgentRansack.exe", "Mozilla Thunderbird email client"));
        self.add(Invocable::bin("email", "shell:::{2559a1f5-21d7-11d4-bdaf-00c04f60b9f0}", "Default email program"));
        self.add(Invocable::bin("slack", "$userpath/AppData/Local/slack/slack.exe", "Slack"));
        self.add(Invocable::bin("sub", "$pf64/Sublime Text 3/sublime_text.exe", "Sublime Text Editor"));
        self.add(Invocable::bin("tb", "$pf86/Mozilla Thunderbird/thunderbird.exe", "Mozilla Thunderbird email client"));
        self.add(Invocable::bin("flp", "$pf64/Mythicsoft/FileLocator Pro/FileLocatorPro.exe", "Agent Ransack file search tool"));
        self.add(Invocable::bin("7z", "$pf64/7-Zip/7zFM.exe", "7-Zip compressed file manager"));
        self.add(Invocable::bin("irfan", "$pf64/IrfanView/i_view64.exe", "IfranView Media Viewer"));
        self.add(Invocable::bin("audacity", "$pf86/Audacity/audacity.exe", "Audacity audio file editor"));
        self.add(Invocable::bin("deskpins", "$pf86/DeskPins/deskpins.exe", "DeskPins"));
        self.add(Invocable::bin("firefox", "$pf64/Mozilla Firefox/firefox.exe", "Mozilla Firefox browser"));
        self.add(Invocable::bin("foobar", "$pf86/foobar2000/foobar2000.exe", "Foobar2000 music player"));
        self.add(Invocable::bin("linqpad", "$pf64/LINQPad6/LINQPad6.exe", "LINQPad for C#"));
        self.add(Invocable::bin("vlc", "$pf86/VideoLAN/VLC/vlc.exe", "VLC Media Player"));
        self.add(Invocable::bin("winmerge", "$pf86/WinMerge/WinMergeU.exe", "WinMerge file and directory comparison tool"));
        self.add(Invocable::bkg("dotpeek", "$userpath/AppData/Local/JetBrains/Installations/dotPeek201/dotPeek64.exe", "JetBrains dotPeek .NET disassembler"));
        self.add(Invocable::bin_with("teams", "$userpath/AppData/Local/Microsoft/Teams/Update.exe", "Microsoft Teams", &["--processStart", "Teams.exe"])); // TODO not working?
        self.add(Invocable::bin("vs", "$pf86/Microsoft Visual Studio/2019/Community/Common7/IDE/devenv.exe", "Microsoft Visual Studio"));
        self.add(Invocable::bin("vscode", "$userpath/AppData/Local/Programs/Microsoft VS Code/Code.exe", "Microsoft Visual Studio Code"));
        self.add(Invocable::bin("rider", "$pf64/JetBrains/JetBrains Rider 2021.1.2/bin/rider64.exe", "JetBrains Rider IDE"));
        self.add(Invocable::bin_with("edge", "$pf86/Microsoft/Edge/Application/msedge.exe", "Microsoft Edge", &["--inprivate", "--ash-force-desktop", "--disable-background-mode", "--disable-preconnect", "--new-window", "--dns-prefetch-disable", "--no-pings", "--process-per-tab", "--no-referrers", "--start-maximized"]));
        self.add(Invocable::bin_with(
            "trackme",
            "$pf86/Microsoft/Edge/Application/msedge.exe",
            "Microsoft Edge",
            &[
                // "--inprivate", // the only difference between command codes edge and trackme
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
