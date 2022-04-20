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
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#running-the-program">Running the Program</a></li>
        <li><a href="#compilation">Compilation</a></li>
        <li><a href="#add-to-path">Add to Path</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
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
fully loaded, meaning when going through them you don't have to await for the page to load like in a browser, it just loads instantly, and the pages are parsed in way to make it easier to view them from the terminal. All the searches are done through DuckDuckGO (only through https), and the requests to the respective pages are done only for getting the html of it and nothing else, making this tool a privacy focused one. For a demo just watch the gif under here !!!INSERT GIF!!!, and for informations about the keybinds go to !!!INSERT KEYBINDS HYPERLINK!!!.

### Supported programming resources (for now, more to come):
#### These are generic resources and work for most languages, in the future i may add language specific ones, but for now this is what I am focusing on.
- [StackOverFlow](https://stackoverflow.com/) don't think I need to say anything about it.
- [StackExchange](https://stackexchange.com/) a q&a forum like stackoverflow that grabs questions and answers from over 170 sources.
- [Github Gists](https://gist.github.com/) here code snippets and guides are posted in form of like a file list.
- [GeeksForGeeks](https://www.geeksforgeeks.org/) can find various good resources on programming and other computer related things.
- [DuckDuckGo Results](https://duckduckgo.com/) as a last resort in case none of the above resources work for you (it's like 20+ of each on every search), you can get the basic sites you get on search and view them, tho it's not gonna be as nicely printed, but still readable so you don't have to leave the terminal

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

1. There are prebuilt binaries in the [releases tab](https://github.com/Obscurely/falion/releases) so you can download those or [compile it yourself](#compilation). So either way get a binary.
*For now there is no like installer to get it in your global path, but go to [add to path](#add-to-path) and follow the steps to manually add it.*

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

### Compilation

This program only uses crossplatform libraries, but I have problems compiling it for windows from linux, when I have time I will spin up a VM to see if it compiles in windows. The following steps require that you have rust installed, check their official [installation page](https://www.rust-lang.org/tools/install).

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
```
cargo build --release
```
It will take a bit depending on your system because of executable size optimizations, but be patient.

4. Done, navigate to target/release and grabe only "falion" file from there.

### Add to path
1. Copy the falion executable to a location you want (it will have to stay there), usually in linux you would create a folder in /opt called falion and put the executable there, or you can place anywhere else in the home dir. As for windows wherever, it's bloated anyway.
2. On linux modify your .zshrc / .bashrc / .fishrc , the hell you use, and add this line to it: (without quotation marks) "alias falion=your/path". On windows you will have to modify your path variable, here is a [guide](https://www.computerhope.com/issues/ch000549.htm).
4. After you are done, you should be able to just type "falion" in cmd and you should see something pop up.

## Usage

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
