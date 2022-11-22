 extern crate serde_json;
 extern crate serde;
 extern crate jsonxf;
 extern crate sys_info;
 extern crate uuid;


 #[macro_use]
 extern crate serde_derive;



pub mod main {

    pub mod runtime_downloader {
        use requests_rs::requests;
        use std::fs::{self};
        use std::path::Path;
        use std::process;
        use whoami::username;

        use crate::main::launcher_cli::is_java_installed;


        /// Downloads the latest jdk runtime for minecraft (currently it is JDK 17)
        ///  As of the moment only windows is supported. I will extend this to linux soon.
        pub fn download_runtime() -> Result<(), std::io::Error>{

            let sys_username = username();

            let home_dir_str = format!("C:\\Users\\{}", sys_username);

            let jdk_download_url = "https://download.bell-sw.com/java/17.0.5+8/bellsoft-jdk17.0.5+8-windows-amd64.msi";

            if cfg!(target_os = "windows") {
                let file_name = Path::new(jdk_download_url).file_stem().unwrap().to_str().unwrap();
                let file_extension = Path::new(jdk_download_url).extension().unwrap().to_str().unwrap();

                if Path::new(&format!("{home_dir_str}/Downloads/{file_name}.{file_extension}")).exists() {
                    println!("Detected msi installer");

                    fs::remove_file(&format!("{home_dir_str}/Downloads/{file_name}.{file_extension}")).expect("Error removing file!");

                    requests::file_downloader::async_download_file(jdk_download_url, &format!("{home_dir_str}/Downloads")).expect("Error downloading file!");

                    println!("Installing java 17");

                    if cfg!(target_os = "windows") {

                        process::Command::new("cmd")
                        .args(["/C", &format!("msiexec /i {home_dir_str}\\Downloads\\{file_name}.{file_extension}")])
                        .spawn().expect("Failed to spawn process").wait().expect("Failed to wait for command");

                        let is_java_installed = is_java_installed();

                        println!("Adding java executable to PATH");

                        process::Command::new("cmd")
                        .args(["/C", &format!("setx PATH C:\\Program Files\\BellSoft\\LibericaJDK-17\\bin ")])
                        .output()
                        .expect("Error adding to PATH");

                        if is_java_installed {
                            println!("Java 17 is installed successfully.")
                        }
                        else {
                            println!("Error installing java 17!");
                            println!("Type java --version in a new prompt to check if it is installed or not.")
                        }
                    }

                }

                else {
                    requests::file_downloader::async_download_file(jdk_download_url, &format!("{home_dir_str}/Downloads")).expect("Error downloading file!");
                    println!("Installing java 17");


                    if cfg!(target_os = "windows") {


                        process::Command::new("cmd")
                        .args(["/C", &format!("msiexec /i {home_dir_str}\\Downloads\\{file_name}.{file_extension}")])
                        .spawn().expect("failed to spawn process").wait().expect("Failed to wait for command");

                        let check_java = process::Command::new("cmd")
                        .args(["/c", "java"])
                        .output()
                        .expect("Error running command");

                        println!("Adding java executable to PATH");

                        process::Command::new("cmd")
                        .args(["/C", &format!("setx PATH C:\\Program Files\\BellSoft\\LibericaJDK-17\\bin ")])
                        .output()
                        .expect("Error adding to PATH");

                        if check_java.status.success() {
                            println!("Java 17 is installed successfully.")
                        }
                        else {
                            println!("Error installing java 17!");
                            println!("Type java --version in a new prompt to check if it is installed or not.")
                        }
                    }

              }

            }

            Ok(())
        }

    }

    pub mod launcher_cli {

        use requests_rs::requests;
        use std::fs::{self, File};
        use std::env;
        use std::path::Path;
        use std::process;
        use uuid::Uuid;
        use minecraft_downloader_core::main::game_downloader::extract_natives;
        use minecraft_downloader_core::main::game_downloader::get_main_class;
        use whoami::username;

        #[derive(Default,Serialize,Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Settings {
            access_token: Option<String>,
            client_token: Option<String>,
            user_info: Vec<Userinfo>,
            pc_info: Vec<PcInfo>,
            minecraft_home: String,
            jvm_args: JvmArgs,
            executable_path: String

        }

        #[derive(Default,Serialize,Deserialize)]
        struct Userinfo {
            username: Option<String>,
            auth_type: Option<String>,
            uuid: Option<String>
            }

        #[derive(Debug,Serialize,Deserialize)]
        struct PcInfo {
            os: String,
            total_ram: String
            }


        #[derive(Default,Serialize,Deserialize)]
        struct JvmArgs {
            args: Vec<Option<String>>
        }

    /// Gets the total system memory in a string.
    pub fn get_total_sys_mem() -> String {
            let total_memory = sys_info::mem_info().unwrap().total;

            let human_readable = (total_memory as f32)/(1e6);

            let string_total_memory = human_readable.to_string()[0..1].to_string();

            string_total_memory
        }

    /// Basic initialization work. All mc launcher related functions should call init() first.
    /// 
    /// Alternatively the init() function can also be called at first time run only. 
    /// 
    /// ```
    ///  use minecraft_launcher_core::main::launcher_cli::init;
    ///  init("minecraft installation directory", refresh_data:true)
    /// ```
    /// If refresh_data is set to true, the settings.json file will be overriden with default values everytime
    pub fn init(install_dir: &str, refresh_data: bool) {

        let sys_username = username();

        let home_dir_str = format!("C:\\Users\\{}", sys_username);

        if refresh_data {
            if Path::new(&format!("{install_dir}/settings.json")).exists()  {
                println!("Detected settings.json at {install_dir}");
                println!("Refreshing the data...");
                let mut settings = Settings::default(); // initialize the struct

                settings.executable_path = format!("{home_dir_str}\\Program Files\\Bellsoft\\LibericaJDK-17\\bin\\java ").to_string();

                settings.minecraft_home = format!("{install_dir}\\.minecraft");

                settings.user_info.push(Userinfo::default()); // push default values

                settings.jvm_args.args.push(None); // push default values

                let total_sys_memory = get_total_sys_mem();

                settings.pc_info.push(PcInfo { os: env::consts::OS.trim().into(), total_ram: format!("{total_sys_memory} GB")});


                let settings_json_str = serde_json::to_string(&settings).unwrap();

                let pretty_json_str = jsonxf::pretty_print(&settings_json_str).unwrap();

                // println!("{pretty_json_str}")

                File::create(format!("{install_dir}/settings.json")).expect("Error creating file!");
                fs::write(format!("{install_dir}/settings.json"), pretty_json_str).expect("Error writing to file!");

                println!("Written new data to {install_dir}/settings.json");

            }
            else {
                let mut settings = Settings::default(); // initialize the struct

                settings.executable_path = format!("{home_dir_str}\\Program Files\\Bellsoft\\LibericaJDK-17\\bin\\java ").to_string();

                settings.minecraft_home = format!("{install_dir}\\.minecraft");

                settings.user_info.push(Userinfo::default()); // push default values

                settings.jvm_args.args.push(None); // push default values

                let total_sys_memory = get_total_sys_mem();

                settings.pc_info.push(PcInfo { os: env::consts::OS.trim().into(), total_ram: format!("{total_sys_memory} GB")});


                let settings_json_str = serde_json::to_string(&settings).unwrap();

                let pretty_json_str = jsonxf::pretty_print(&settings_json_str).unwrap();

                // println!("{pretty_json_str}")

                File::create(format!("{install_dir}/settings.json")).expect("Error creating file!");
                fs::write(format!("{install_dir}/settings.json"), pretty_json_str).expect("Error writing to file!");

                println!("Written data to {install_dir}/settings.json");
            }
        }

        else {
            if Path::new(&format!("{install_dir}/settings.json")).exists()  {
                println!("Detected settings.json at {install_dir}, ignoring...");

            }
            else {
                let mut settings = Settings::default(); // initialize the struct

                settings.executable_path = format!("{home_dir_str}\\Program Files\\Bellsoft\\LibericaJDK-17\\bin\\java ").to_string();

                settings.minecraft_home = format!("{install_dir}\\.minecraft");

                settings.user_info.push(Userinfo::default()); // push default values

                settings.jvm_args.args.push(None); // push default values

                let total_sys_memory = get_total_sys_mem();

                settings.pc_info.push(PcInfo { os: env::consts::OS.trim().into(), total_ram: format!("{total_sys_memory} GB")});

                let settings_json_str = serde_json::to_string(&settings).unwrap();

                let pretty_json_str = jsonxf::pretty_print(&settings_json_str).unwrap();

                // println!("{pretty_json_str}")

                File::create(format!("{install_dir}/settings.json")).expect("Error creating file!");
                fs::write(format!("{install_dir}/settings.json"), pretty_json_str).expect("Error writing to file!");

                println!("Written data to {install_dir}/settings.json");
            }
        }


        }


    /// Checks whether java is installed on the system or not
    /// 
    /// Checks only for windows at the moment
    pub fn is_java_installed() -> bool {
        if cfg!(target_os="windows") {
            let check_java = process::Command::new("cmd")
            .args(["/C", "java --version"])
            .output()
            .expect("Error checking for java");

            if check_java.status.success() {
                true
            }
            else {
                false
            }
        }

        else {
            println!("Linux/MacOS not implemented yet");
            false
        }
         
    }

    // The struct of the data from the GET request.

    #[derive(Debug,Serialize,Deserialize)]
    struct ElyGetResponse {
        id:String,
        name:String
        }

    // The struct of the data to be POSTed

    #[derive(Debug,Serialize,Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ElySendPost {
        username: String,
        password: String,
        client_token: String,
        request_user: bool
        }

    // The struct of the response coming after the POST request

    #[derive(Debug,Serialize,Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ElyApiResponse {
            access_token: String,
            available_profiles: Vec<AvailableProfiles>,
            client_token: String,
            selected_profile: SelectedProfile,
            user: User

        }

    #[derive(Debug,Serialize,Deserialize)]
    struct AvailableProfiles{
            id: String,
            name: String,
        }

    #[derive(Debug,Serialize,Deserialize)]
    struct SelectedProfile {
            id: String,
            name: String,
        }

    #[derive(Debug,Serialize,Deserialize)]
    struct User {
            id: String,
            properties: Vec<Properties>,
            username: String
        }

    #[derive(Debug,Serialize,Deserialize)]
    struct Properties {
            name: String,
            value: String
        }


    /// Does user authentication at www.ely.by
    pub fn ely_by_authenticator(user_name: &str, password: &str, install_dir: &str) {

        let sys_username = username();

        let home_dir_str = format!("C:\\Users\\{}", sys_username);

        let check_user = requests::api_referencer::sync_get(&format!("https://authserver.ely.by/api/users/profiles/minecraft/{user_name}"));

        if check_user.status().is_success() {
            println!("User found, proceeding to authentication...");

            let request_data = requests::api_referencer::get_and_save_json(&format!("https://authserver.ely.by/api/users/profiles/minecraft/{user_name}"), false, true).expect("Error getting data!");

            let ely_get_response: ElyGetResponse = serde_json::from_value(request_data).unwrap();

            let client_token_generated = Uuid::new_v4().to_string(); // random uuid everytime.

            let ely_send_post = ElySendPost {
                username: ely_get_response.name,
                password: password.to_string(),
                client_token: client_token_generated,
                request_user: true
            };

            let generated_json = serde_json::to_string(&ely_send_post).unwrap();

            let pretty_json_str = jsonxf::pretty_print(&generated_json).unwrap();


            if cfg!(target_os = "windows") {

                // save the data to be posted.

                File::create(format!("{home_dir_str}/AppData/Roaming/post.json")).expect("Error creating file!");
                fs::write(format!("{home_dir_str}/AppData/Roaming/post.json"), pretty_json_str).expect("Error writing to file!");

                // make the post request
                let auth_response = requests::api_referencer::print_and_post_json("https://authserver.ely.by/auth/authenticate", &format!("{home_dir_str}\\AppData\\Roaming\\post.json"), true).expect("Error posting data!");

                // save it to settings.json

                let api_response: ElyApiResponse = serde_json::from_value(auth_response).unwrap();

                let access_token = api_response.access_token;

                let total_sys_memory = get_total_sys_mem();


                let settings = Settings{
                    access_token: Some(access_token),
                    client_token: Some(ely_send_post.client_token),
                    user_info: vec![Userinfo {
                        username: Some(user_name.into()),
                        auth_type: Some("ely_by".into()),
                        uuid: Some(ely_get_response.id)
                    }],
                    pc_info: vec![PcInfo {
                        os: env::consts::OS.trim().into(),
                        total_ram: format!("{total_sys_memory} GB").into()
                    }],
                    minecraft_home: format!("{install_dir}\\.minecraft"),
                    jvm_args: JvmArgs { args: vec![None] },
                    executable_path: format!("{home_dir_str}\\Program Files\\Bellsoft\\LibericaJDK-17\\bin\\java").into()
                    };


                let settings_str = serde_json::to_string(&settings).unwrap();

                let settings_str_pretty = jsonxf::pretty_print(&settings_str).unwrap();

                update_settings(&settings_str_pretty, install_dir);

                }
            }
            else {
                println!("The requsted user does not exist.")
            }


        }

        pub fn update_settings(data: &str, install_dir: &str) {

            fs::write(format!("{install_dir}\\settings.json"), data).expect("Error writing to file!");
            println!("Updated {install_dir}\\settings.json")

        }

        pub fn download_ely_authlib(install_dir: &str) {
            let download_url = "https://github.com/yushijinhun/authlib-injector/releases/download/v1.2.1/authlib-injector-1.2.1.jar";

            
            if !Path::new(&format!("{}\\authlib", install_dir)).exists() {
                fs::create_dir(&format!("{}\\authlib", install_dir)).expect("Error creating directory!");
                requests::file_downloader::async_download_file(download_url, &format!("{}\\authlib", install_dir)).expect("Error downloading file!")

            }
            
            else {
                requests::file_downloader::async_download_file(download_url, &format!("{}\\authlib", install_dir)).expect("Error downloading file!")
            }

        }


        #[derive(Debug)]
        struct LaunchOptions {
            username: String,
            uuid: String,
            token: String,
        }

        /// Launches minecraft for the specified game_version, it's type(release or snapshot) and the authentication type used.
        /// 
        /// Supports www.ely.by authentication for now
        /// 
        /// Supports only Windows at the moment. I will soon extend this to linux as well.
        /// 
        /// ```
        /// use minecraft_launcher_core::launcher_cli::launch_mc_vanilla;
        /// 
        /// launch_mc_vanilla(auth_type:"ely_by", game_version:"1.19.2", r#type: "release/snapshot", username: "your username on ely.by", password: "your password on ely.by", alloc_mem: "2")
        /// 
        /// ```
        /// alloc_mem is the amount of memory you want to allocate to minecraft (It takes whole numbers for GB only)
        /// 
        pub fn launch_mc_vanilla(auth_type: &str, game_version: &str, r#type: &str, install_dir: &str, username: &str, password: &str, alloc_mem: &str) {
            if auth_type == "ely_by".trim() && cfg!(target_os="windows") {

                download_ely_authlib(install_dir);

                ely_by_authenticator(username, password, install_dir);

                extract_natives(game_version, install_dir, &format!("{install_dir}\\.minecraft\\versions\\{game_version}\\"), "windows");

                let settings_data = fs::read_to_string(format!("{install_dir}\\settings.json")).expect("Error reading from file");

                let mut settings: Settings = serde_json::from_str(&settings_data).expect("Error reading json data!");

                let jvm_args_list = vec![format!("-javaagent:{}/authlib/authlib-injector-1.2.1.jar=ely.by", install_dir), format!("-Xmx{alloc_mem}G"), format!("-Xms128m"), "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump".to_string() ];

                settings.jvm_args.args.clear();

                for arg in jvm_args_list.iter() {
                    settings.jvm_args.args.push(Some(arg.clone()))
                }

                let settings_str = serde_json::to_string(&settings).unwrap();

                let settings_str_pretty = jsonxf::pretty_print(&settings_str).unwrap();

                update_settings(&settings_str_pretty, install_dir);

                let launch_options = LaunchOptions {
                    username: settings.user_info[0].username.as_ref().unwrap().clone(), //as_ref() takes a reference to the contents of the vector,clone() gets the original value
                    uuid: settings.user_info[0].uuid.as_ref().unwrap().clone(),
                    token: settings.access_token.as_ref().unwrap().clone(),
                };
                    

                let mut classpath = minecraft_downloader_core::main::game_downloader::get_class_path(install_dir, game_version);

                classpath+=&format!("{}\\.minecraft\\versions\\{}\\{}.jar", install_dir, game_version, game_version);

                let main_class = get_main_class(game_version, install_dir);

                let cmd_arr = [
                    "/C",
                    "java", 
                    &jvm_args_list[0], 
                    &jvm_args_list[1], 
                    &jvm_args_list[2], 
                    &jvm_args_list[3], 
                    "-Dos.name=Windows 10",
                    "-Dos.version=10.0", 
                    &format!("-Djava.library.path={install_dir}\\.minecraft\\versions\\{game_version}\\natives"), 
                    "-Dminecraft.launcher.brand=minecraft-launcher-core", 
                    "-Dminecraft.launcher.version=0.1.0", 
                    "-cp",
                    &format!("{}", classpath),
                    &format!("{}", main_class), 
                    "--username",
                    &format!("{}", launch_options.username), 
                    "--version",
                    &format!("{}", game_version), 
                    "--gameDir",
                    &format!("{}\\.minecraft", install_dir), 
                    "--assetsDir",
                    &format!("{}\\.minecraft\\assets", install_dir), 
                    "--assetIndex",
                    &format!("{}", &game_version[0..4]),
                    "--uuid",
                    &format!("{}", launch_options.uuid), 
                    "--accessToken",
                    &format!("{}", launch_options.token), 
                    "--clientId",
                    &format!("{}", &settings.client_token.unwrap()), 
                    "--xuid",
                    &format!("${{auth_xuid}}"), 
                    "--userType",
                    "mojang", 
                    "--versionType",
                    &format!("{}", r#type)];
                    

                println!("Launching minecraft version {}", game_version);

                process::Command::new("cmd")
                .args(cmd_arr)
                .spawn().expect("failed to spawn process").wait().expect("Failed to wait for command");

            }
        }


    }
}

