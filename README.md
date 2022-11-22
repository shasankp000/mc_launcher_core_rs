# mc_launcher_core_rs
A minecraft launcher library written in rust

https://crates.io/crates/minecraft_launcher_core

# Platforms supported 

Since this library uses https://github.com/shasankp000/mcdl_core_rs/ as an underlying dependency, which only supports windows at the moment, this library(all functions in this library) support only windows at the moment as well.

However support will be extended to Linux and MacOS soon.

# Note 

To launch mc, you need to have an account at www.ely.by since the launcher lib supports user authentication from www.ely.by only as of the moment

# Functions 

# download_runtime

Downloads the latest jdk runtime for minecraft (currently it is JDK 17)
As of the moment only windows is supported. I will extend this to linux soon.

```minecraft_launcher_core::main::runtime_downloader::download_runtime;```

```download_runtime()```

## is_java_installed

Checks whether java is installed on the system or not

Checks only for windows at the moment

```minecraft_launcher_core::main::launcher_cli::is_java_installed`;``

``` let is_java_installed = is_java_installed();```

``` println!("{}", is_java_installed)``` (true/false)

## get_total_sys_mem

Gets the total system memory in a string.

```minecraft_launcher_core::main::launcher_cli::get_total_sys_mem;```

``` let total_mem = get_total_sys_mem();```

```println!("{}", total_mem)``` 

For 2GB, it is "2", 4GB, it is "4", 8GB, it is "8" and so on..

# init()

Basic initialization work. All mc launcher related functions should call init() first.
 
Alternatively the init() function can also be called at first time run only. 

```use minecraft_launcher_core::main::launcher_cli::init;```
```init("minecraft installation directory", refresh_data:true)```

If refresh_data is set to true, the settings.json file will be overriden with default values everytime

## ely_by_authenticator

Does user authentication at www.ely.by and saves the data to settings.json

```use minecraft_launcher_core::main::launcher_cli::ely_by_authenticator;```

```ely_by_authenticator("your username at ely.by", "your password at ely.by", "minecraft_installation_directory")```

## download_ely_authlib

Downloads the ely_by authlib

``` use minecraft_launcher_core::main::launcher_cli::download_ely_authlib; ```

``` download_ely_authlib("minecraft installation directory")```

# launch_mc_vanilla

Launches minecraft for the specified game_version, it's type(release or snapshot) and the authentication type used.

This function has only been tested with "release" versions of minecraft. Snapshot support will be added in the next release in both the underlying dependency and this library.

Supports www.ely.by authentication for now

Supports only Windows at the moment. I will soon extend this to Linux and MacOS as well.

```use minecraft_launcher_core::launcher_cli::launch_mc_vanilla;```
 
```launch_mc_vanilla(auth_type:"ely_by", game_version:"1.19.2", r#type: "release/snapshot", username: "your username on ely.by", password: "your password on ely.by", alloc_mem: "2")```


alloc_mem is the amount of memory you want to allocate to minecraft (It takes whole numbers for GB only)


