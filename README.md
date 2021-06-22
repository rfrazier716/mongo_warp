<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the mongo_warp and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Thanks again! Now go create something AMAZING! :D
***
***
***
*** To avoid retyping too much info. Do a search and replace for the following:
*** rfrazier716, mongo_warp, twitter_handle, email, Zero To Production, project_description
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/rfrazier716/mongo_warp">
    <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" alt="Logo" width="200">
  </a>

<h3 align="center">Mongo Warp</h3>

  <p align="center">
    An Async Warp Webserver Template With MongoDB Connection
    <br />
    <a href="https://github.com/rfrazier716/mongo_warp"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/rfrazier716/mongo_warp">View Demo</a>
    ·
    <a href="https://github.com/rfrazier716/mongo_warp/issues">mongo_warprt Bug</a>
    ·
    <a href="https://github.com/rfrazier716/mongo_warp/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
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
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

More details coming soon!

In short, this is a Rust webserver template using Warp and MongoDB 
to help speed up API development.

### Features
* Turnkey Asynchronous Webserver with runtime configuration through config files and environment variables.
* Cached CI/CD pipeline with CircleCI including the following jobs:
    * Formatting check with `cargo fmt`
    * Linting Check with `cargo clippy`
    * vulnerability check with `cargo audit`
    * unittests run on every commit
    * Integration tests with database queries that run on commits to `main` or `development`
* Built with the warp framework, which builds on top of hyper.
* All the security benefits of Rust

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites
It's assumed you already have Rust installed on your computer as well as cargo, follow their 
[installation instructions](https://www.rust-lang.org/tools/install) for OS specific installation guides. 


* cargo-generate
  ```sh
  cargo install cargo-generate
  ```
* [Docker](https://docs.docker.com/get-docker/) -- Required to run integration tests

### Installation

1. Install the project template using Cargo Generate. Note you need to install from the `template` branch.
   ```sh
   cargo generate --git https://github.com/rfrazier716/mongo_warp.git --branch template
   ```
   
2.  Answer the additional prompts for project names and repository details:
    * `Project Name`: This will be the name of the executable and crate. Any hyphens will be converted to underscores code to ensure valid crate names.
    * `github username`: Is used to generate shields and hyperlinks in the readme
    * `repository name`: Is used to generate shields and hyperlinks in the readme
    * `LinkedIn Shield`: If set to true, a LinkedIn shield will be included at the top of the repository, otherwise ignored.
    
<!-- USAGE EXAMPLES -->
This repository is a template to get you up and running with a Rust Webserver faster. For that reason it only includes a basic `/health`
route that pings the MongoDB instance. To add additional routes and endpoints see the documentation for [Warp](https://docs.rs/warp/0.3.1/warp/) and [MongoDB](https://github.com/mongodb/mongo-rust-driver).

For an example of the template in action, see my [warp_crud](https://github.com/rfrazier716/warp_crud) repository.

### Running Tests

The Template includes both unit-tests and integration tests to demonstrate:
1. Testing routes using Warp without starting the server
2. Running async integration tests on the first available port to allow for parallelization

#### Unit Tests
To run unit tests navigate into the project root directory and run
```shell
cargo test --lib
```

#### Integration Tests
For integration tests you'll need to spin up a MongoDB container on port 27017. This is handled by docker and the
`docker-compose.yml` file in `/tests`.

```shell
cd tests
docker compose up
```
In addition to the MongoDB image, A [mongo-express](https://github.com/mongo-express/mongo-express) instance will also
start on  `localhost:8081`.

Once the container is running you can run both Integration and unit tests with Cargo.

```shell
cargo test
```

### Starting the Server
To launch the web server navigate to the base directory and run. The same MongoDB instance used for health checks must be running or the health check will timeout.
```shell
cargo run
```

The default configuration will start the server on `localhost:3030`. You can check the health endpoint is running with `curl`:
```shell
$ curl -i localhost:3030/health
HTTP/1.1 200 OK
content-length: 0
date: Tue, 22 Jun 2021 16:57:23 GMT
```

<!-- ROADMAP -->
## Roadmap

See the [open issues](https://github.com/rfrazier716/mongo_warp/issues) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License
Licensed under Apache v2.0


<!-- CONTACT -->
## Contact

Ryan Frazier - [@fotonixandgizmo](https://twitter.com/fotonixandgizmo) - Ryan@Fotonixx.com

Project Link: [https://github.com/rfrazier716/mongo_warp](https://github.com/rfrazier716/mongo_warp)



<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [Zero to Production in Rust](https://www.zero2prod.com/)
* [Warp web server Framework](https://github.com/seanmonstar/warp)
* [best-README-template](https://github.com/othneildrew/Best-README-Template)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/rfrazier716/mongo_warp.svg?style=for-the-badge
[contributors-url]: https://github.com/rfrazier716/mongo_warp/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/rfrazier716/mongo_warp.svg?style=for-the-badge
[forks-url]: https://github.com/rfrazier716/mongo_warp/network/members
[stars-shield]: https://img.shields.io/github/stars/rfrazier716/mongo_warp.svg?style=for-the-badge
[stars-url]: https://github.com/rfrazier716/mongo_warp/stargazers
[issues-shield]: https://img.shields.io/github/issues/rfrazier716/mongo_warp.svg?style=for-the-badge
[issues-url]: https://github.com/rfrazier716/mongo_warp/issues
[license-shield]: https://img.shields.io/github/license/rfrazier716/mongo_warp.svg?style=for-the-badge
[license-url]: https://github.com/rfrazier716/mongo_warp/blob/master/LICENSE.txt
