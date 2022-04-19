<div id="top"></div>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/Obscurely/falion">
    <img src="resources//logo.png" alt="Logo" width="460" height="123">
  </a>

  <h3 align="center">Falion</h3>

  <p align="center">
    
    <br />
    <a href="https://github.com/Obscurely/falion/issues">Report Bug</a>
    |
    <a href="https://github.com/Obscurely/falino/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#running-the-program">Running the Program</a></li>
        <li><a href="#compilation">Compilation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li>
      <a href="#acknowledgments">Acknowledgments</a>
      <ul>
        <li><a href="#the-difference-between-the-2-startup-options">The difference between the 2 startup options</a></li>
      </ul>
    </li>
  </ol>
</details>

## About The Project

A ransomware type virus made for learning purposes and learning purposes only, any damage that this virus causes to your computer I don't take responsability for it.
It sorts the files of the pc in order of importance and encrypts them using AES-128 (hasn't been cracked), the files classed as non important (such as program and games) get deleted making the encryption process quite fast.
The ransomware also modifes the system making it more "locked down" by disabling features of windows such as control panel, run, etc.
The ransomware injects it's self on startup using one of the methods (a. using alternate data streams | b.classic shortcut run, check [the difference between the two](#the-difference-between-the-2-startup-options) to find more) and resumes the process where it left off and encrypts any new file.
The ransomware can be ran even without admin and will encrypt any file it can. (you need to modify the manifest for this to remove admin)

### Built with

- [Rust 1.58.0](https://www.rust-lang.org/)

#### The stock libraries and these awesome 3rd party ones:
- [aes-gcm-siv](https://lib.rs/crates/aes-gcm-siv) for encryption
- [rand](https://lib.rs/crates/rand) for basic random number generation
- [rand_hc](https://lib.rs/crates/rand_hc) for cryptographically secure random number generation
- [fs_extra](https://lib.rs/crates/fs_extra) for extra file system operations
- [dirs](https://lib.rs/crates/dirs) for getting common paths like Downloads folder
- [walkdir](https://lib.rs/crates/walkdir) for recursively reading a dir
- [mountpoints](https://lib.rs/crates/mountpoints) for getting a list of all mounted drives
- [winreg](https://lib.rs/crates/winreg) for editing registry keys in windows
- [mslnk](https://lib.rs/crates/mslnk) for creating windows shortcuts
- [embed-manifest](https://lib.rs/crates/embed-manifest) for editing exe manifest automatically on compile in order for it to require admin by default.

## Getting Started

### Running The Program

*I am not gonna give you precompiled binaries so any 7 year old can run this on their dad's computer.*

1. After compling it (check [compilation](#compilation)) simply spin up a windows vm (or a real machine...), double click the exe say yes to admin and enjoy.

### Compilation

This ransomware is made for windows only and will only compile for windows. The following steps require that you have rust installed, check their official [installation page](https://www.rust-lang.org/tools/install). Before going ahead and compiling I recommend that you at least look at the main.rs file in src folder and take a look at the 3 vars at the top and change them accordingly (also take a look at [the difference between the 2 startup options](#the-difference-between-the-2-startup-options). Also idealy you would want to check the source code for any folder names and other stuff to make the ransomware more foreign.

1.  Clone this repo on your pc, you can use "git clone", if you have git installed, like this:

```
git clone https://github.com/Obscurely/falion.git
```

Otherwise in the right up side of the repo page you will see a download button, download the repo as zip and extract it in a folder

2.  Open a new terminal/cmd window in the folder you extracted the repo in, if you can't rightclick on the folder and open it there do: 

On Linux and windows:
```
cd the/path
```
and you will get there.

3.  From there run this compile command in the terminal:

On Linux (this would require installing this specific toolchain, check [this](https://www.rust-lang.org/tools/install) out for more information):
```
cargo build --target x86_64-pc-windows-gnu --release
```
On windows:
```
cargo build --release
```

4.  Your build is gonna be in the target/x86_64-pc-windows-gnu/release in Linux and in windows in target/release. You only need the exe not all the files.

## Usage

Education purposes only. If you don't know how to spin up a VM to test this, check out virtual box as is the easiest. If you use linux and already know your way around VMs and didn't ever use qemu/kvm I recommend you check it out.

## Roadmap

Nothing really. Maybe if I think of anything else that can be improved I will add it, as for now bug fixes only.

## Contributing

Edit a file you want, do a [pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request), I will look at it and if the change makes sense and is a good one I will accept it and that's it.

## License

Is under [MIT license](https://mit-license.org/) and I ain't gonna care whatever anyone does with the code, so stick to the license conditions and have fun :)

## Contact

Either post an issue in the [Issues Tab](https://github.com/Obscurely/falion/issues) or contact me at this email adddress if you have more to say: obscurely.social@protonmail.com

## Acknowledgments
### The difference between the 2 startup options
The classic method simply creates a folder and shortcut of the exe in the appdata folder, and that shortcut is added to a reg key to be ran on boot.
The other method uses alternate that streams if possible, if not it falls back to the classic method. The method for the startup can be selected by modifying the value of the boolean in the code, it's currently set to the alternate data stream method.

An alternate data stream is a feature in windows that basically is used for compatibility purposes and directions for how smart screen should work, this data can be on a file or a folder. You can't see the data stored in an alternate data stream in file explorer even if you have show protected system files checked. We abuse this feature and copy the ransomware into a folder's alternate data stream called default_id for hidding purposes on a folder in appdata called Cache (of course you would change this if you plan to use this maliciously) and make a special startup command in registry to extract the ransomware from this folder and execute it.

This method is way more hidden and harder to be detected by the user or an anti virus removal tool, but has it's drawbacks. First a command prompt will show on boot for a bare second which no matter how hard i tried I can't alt f4 it, but still it's a potential risk to not execute the ransomware on boot, tho I don't think it's possible to actually catch the window. The second draw back is that the run command in registry is limited to a number of characters and using the method it's using now to execute on boot (can be shortened but is the way it is in order for the command prompt the stay as little as possible on the screen) makes it so if the user uses the maximum number of chars for his username which is 20 then if the ransom file name is more than 20 chars it would break the startup command since it's too long. Now if you name your exe more than 20 chars by a bit you should still be ok since most people don't have 20 char usernames, but to be safe don't go above 20.

Now onto when to use which method, it depends on the user you are attacking. If you are attacking a wider variety of people then use the classic method if you target a specific group of people use the alternate data stream method or this method regardless, but for now it's still in testing so who know what I may found out using this method.

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/Obscurely/falion.svg?style=for-the-badge
[contributors-url]: https://github.com/Obscurely/falion/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Obscurely/falion.svg?style=for-the-badge
[forks-url]: https://github.com/Obscurely/falion/network/members
[stars-shield]: https://img.shields.io/github/stars/Obscurely/falion.svg?style=for-the-badge
[stars-url]: https://github.com/Obscurely/falion/stargazers
[issues-shield]: https://img.shields.io/github/issues/Obscurely/falion.svg?style=for-the-badge
[issues-url]: https://github.com/Obscurely/falion/issues
[license-shield]: https://img.shields.io/github/license/Obscurely/falion.svg?style=for-the-badge
[license-url]: https://github.com/Obscurely/falion/blob/master/LICENSE
