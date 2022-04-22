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
    An open source, programmed in rust, privacy focused tool for reading programming resources (like stackoverflow) fast,
efficient and asynchronous from the terminal.
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
        <li><a href="supported-programming-sources">Supported programming sources</a></li>
        <li><a href="#video-showcase">Video showcase</a></li>
        <li><a href="#built-with">Built with</a></li>
        <ul>
          <li><a href="#the-stock-libraries-and-these-awesome-3rd-party-ones">The stock libraries and these awesome 3rd party ones</a></li>
        </ul>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#running-the-program">Running the Program</a></li>
        <ul>
          <li><a href="#install-with-cargo">Install with cargo</a></li>
          <li><a href="#install-from-aur">Install from AUR</a></li>
          <li><a href="#install-from-provided-binaries">Install from provided binaries</a></li>
          <li><a href="#manually">Manually</a></li>
        </ul>
        <li><a href="#compilation">Compilation</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#basics">Basics</a></li>
        <li><a href="#key-binds">Key Binds</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

## About The Project

An open source, programmed in rust, privacy focused tool for reading programming resources (like stackoverflow) fast,
efficient and asynchronous from the terminal. By the time the results appear most of the pages are 
fully loaded, meaning when going through them you don't have to await for the page to load like in a browser, it just loads instantly, and the pages are parsed in way to make it easier to view them from the terminal. All the searches are done through DuckDuckGO (only through https), and the requests to the respective pages are done only for getting the html of it and nothing else, making this tool a privacy focused one. For a demo just watch the video under here, and for informations about the keybinds go to [key binds](#key-binds)

### Supported programming sources:
**These are generic resources and work for most languages, in the future i may add language specific ones, but for now this is what I am focusing on. More will come.**
- [StackOverFlow](https://stackoverflow.com/) don't think I need to say anything about it.
- [StackExchange](https://stackexchange.com/) a q&a forum like stackoverflow that grabs questions and answers from over 170 sources.
- [Github Gists](https://gist.github.com/) here code snippets and guides are posted in form of like a file list.
- [GeeksForGeeks](https://www.geeksforgeeks.org/) can find various good resources on programming and other computer related things.
- [DuckDuckGo Results](https://duckduckgo.com/) as a last resort in case none of the above resources work for you (it's like 20+ of each on every search), you can get the basic sites you get on search and view them, tho it's not gonna be as nicely printed, but still readable so you don't have to leave the terminal

### Video showcase

https://user-images.githubusercontent.com/59087558/164259195-416191d5-07c6-4b7d-8af6-aecb3ef53537.mp4

### Built with

- [Rust 1.60.0](https://www.rust-lang.org/)

#### The stock libraries and these awesome 3rd party ones:
- [reqwest](https://lib.rs/crates/reqwest) for making all the https requests.
- [tokio](https://lib.rs/crates/tokio) for making requests asynchronously.
- [regex](https://lib.rs/crates/regex) for scraping information about pages, like getting urls etc.
- [urlencoding](https://lib.rs/crates/urlencoding) for encoding the query in order to be url compliant.
- [futures](https://lib.rs/crates/futures) for handling the asynchronous tasks.
- [colored](https://lib.rs/crates/colored) for easily colorizing the terminal output.
- [crossterm](https://lib.rs/crates/crossterm) for manipulating the terminal, like getting key inputs, clearing it and others.
- [html2text](https://lib.rs/crates/html2text) for converting html to readable text in order to display pages in terminal better.
- [url](https://lib.rs/crates/url) for parsing strings to an url object for easier and safer manipulation.
- [indexmap](https://lib.rs/crates/indexmap) for having an object HashMap like that supports indexing.
- [argparse](https://lib.rs/crates/argparse) to easily handle command line arguments.

## Getting Started

### Running The Program

#### Install with cargo
This method will work across most (if not all) linux distributions supporting cargo, and other operating systems that support rust and cargo (I only tested on linux).
1. Install rust, cargo and all it's things using the official [rustup installer](https://www.rust-lang.org/tools/install)
2. Run the following command in your terminal of choice:
```
cargo install falion
```
3. Make sure you have .cargo/bin in path, you would need to add the following line in your terminal rc file (e.g $HOME/.zshrc)
```
export PATH=$HOME/.cargo/bin:$PATH
```
On windows it should work automatically (restart if just installed), if not you can follow this [guide](https://www.computerhope.com/issues/ch000549.htm) for how to add something to path. The cargo bin folder will be {your-user-folder}\\.cargo\\bin

4. In order to update run the install command again, and you can now follow [usage](#usage) for more information on how to use it.

#### Install from AUR
a. Using yay or any other AUR helper
```
yay -Sy falion-bin
```
b. Manually cloning and building it from AUR
  1. First install the basic build dependencies, if you don't already have them:
  ```
  sudo pacman -Sy gcc base-devel --needed
  ```
  2. Then clone the build script
  ```
  git clone https://aur.archlinux.org/falion-bin.git
  ```
  3. Cd into the new cloned repo and run the following to build the package
  ```
  makepkg
  ```
  4. In order to install the package run the following (where * is just an any other characters place holder)
  ```
  sudo pacman -U falion-bin-*.pkg.tar.zst
  ```

#### Install from provided binaries
a. For Arch Linux based distros (not recomended, use aur in order to have auto updates aswell)
  1. Download from the [releases tab](https://github.com/Obscurely/falion/releases/) from the version you want (latest stable recommended), the file named like falion-bin-\*.pkg.tar.zst
  2. From where you downloaded it run the following command in your terminal of choice (where * is just an any other characters place holder):
  ```
  sudo pacman -U falion-bin-*.pkg.tar.zst
  ```
b. For Debian based distros (I'm working on a ppa, for now I recomended you use the cargo version insted)
  1. Download from the [releases tab](https://github.com/Obscurely/falion/releases/) from the version you want (latest stable recommended), the file named like falion_\*\_debian_amd64.deb
  2. From where you downloaded it run the following command in your terminal of choice (where * is just an any other characters place holder):
  ```
  sudo dpkg -i falion_*_debian_amd64.deb
  ```

#### Manually
Placing the executable somewhere than adding it to path. (Not recomended, [installing it with cargo](#install-with-cargo) is better)
1. Either follow [compilation](#compilation) and build it for the platform of your choice or download from the [releases tab](https://github.com/Obscurely/falion/releases/) the prebuilt linux binary, called "falion"
2. Copy the falion executable to a location you want (it will have to stay there), usually in linux you would create a folder in /opt called falion and put the executable there, or you can place anywhere else in the home dir.
3. On linux modify your .zshrc / .bashrc / .fishrc , the hell you use, and add this line to it: (without quotation marks) "alias falion=your/path". On windows you will have to modify your path variable, here is a [guide](https://www.computerhope.com/issues/ch000549.htm). And on Mac same as Linux.
4. After you are done, you should be able to just type "falion" in terminal and you should see something pop up, saying you didn't input any query and directing you to run falion -h.

### Compilation

This program only uses crossplatform libraries, but I have problems compiling it for windows from Linux, when I have time I will spin up a VM to see if it compiles in windows (on macos it should like 99.99% compile without problems). The following steps require that you have rust installed, check their official [installation page](https://www.rust-lang.org/tools/install).

1.  Clone this repo on your pc, you can use "git clone", if you have git installed, like this:
```
git clone https://github.com/Obscurely/falion.git
```
Otherwise in the right up side of the repo page you will see a download button, download the repo as zip and extract it in a folder

2.  Open a new terminal/cmd window in the folder you extracted the repo in, if you can't rightclick on the folder and open it there do:
```
cd the/path
```
and you will get there.

3.  From there run this compile command in the terminal:
```
cargo build --release
```
It will take a bit depending on your system because of executable size optimizations, but be patient.

4. Done, navigate to target/release and grab only the "falion" file from there. Now you can follow [manually](#manually) install

## Usage

### Basics
1. First you would have to get it installed and in path, follow [this](#getting-started), after you can continue.
2. Then from the terminal (regardless of the os) you can use it by running these commands. <div></div>
For getting help about the program
```
falion -h
```
For getting a list of the keybinds, also available on this readme at [key binds](#key-binds)
```
falion -k
```
For doing a search
```
falion rust how to print
```
Or if you want to do a search and see all the warnings (like parsing problems of text etc) run it in verbose mode
```
falion -v rust how to print
```

### Key binds

#### Key Binds list for falion!
**Note: where '..' is used it means from that to that like '1..5' would mean from 1 to 5.**

#### Main menu:
**[1..5]**         = Access that resource.<br />
**SHIFT + [1..5]** = Go to the next element in the list of that resource.<br />
**ALT + [1..5]**   = Go to the previous element in the list of that resource.<br />
**CTRL + n**       = Move to the next element in the list of every resource.<br />
**CTRL + b**       = Move back to the previous element in the list of every resource.<br />
**CTRL + c**       = Clear terminal and exit.<br />

#### Sub menus for the resources:
**CTRL + n**       = Move to the next element in the content list (like questions & answers).<br />
**CTRL + b**       = Move back to the previous element in the content list.<br />
**CTRL + q**       = Go back to the main menu.<br />
**CTRL + c**       = Clear terminal and exit.<br />

#### These were all the key binds, enjoy using Falion!

## Roadmap

Adding more generic resources, but also maybe add lanaguage related one that get enabled based on the first word in the query. And also just improve it in general.

## Contributing

Edit a file you want, do a [pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request), I will look at it and if the change makes sense and is a good one I will accept it and that's it.

## License

Is under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html) so stick to the license conditions and have fun :)

## Contact

Either post an issue in the [Issues Tab](https://github.com/Obscurely/falion/issues) or contact me at this email adddress if you have more to say: obscurely.social@protonmail.com

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
