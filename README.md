# DSP Seed Finder

DSP Seed Finder is a tool designed to assist you in finding seeds for Dyson Sphere Program.

[Link to DSP Seed Finder](https://doubleuth.github.io/DSP-Seed-Finder/).

This tool offers two primary functions. The star finder enables you to search for star systems that meet specific criteria tailored to your preferences. The galaxy viewer provides a overview of galaxy details, allowing you to view different galaxies in the game.

This tool is not official and is not affiliated with the game developers.

## Notes

This tool runs on your own PC instead of a server, which means it will utilize your CPU and slow down other processes during the search process.

The search results are automatically stored in your browser, allowing you to pause the search at any time and resume it later. However, please note that clearing your browser history may also delete the search results.

This tool offers two searching modes. By default, it operates in browser mode, conducting the search within your browser. Alternatively, it can run in native mode, requiring you to download and run a program locally for the search to take place on your machine. In my experience, native mode runs 2-5 times faster. Browser mode is provided for users who may have concerns about downloading programs.

This tool does not accurately generate the amount of vein a planet has. The reason for this is that the vein generation algorithm in the game is extremely complex and relies on specific functions within the Unity Engine. Implementing this algorithm accurately in this tool is very challenging and time-consuming. Therefore, an estimated value for the vein amount is provided as an alternative.

If you find the search process to be slow, you can try narrowing it down by applying more specific rules. The tool is optimized for finding rare, one-in-a-million kind of seeds.

## Reporting Issues

If you run into any issues while using this tool, feel free to open a new issue in this repository.

## Compile the program

1. Install Rust and Node.js
2. Run the following command to install wasm-pack on your machine.

```shell
cargo install wasm-pack
```

3. Run the following to install dependencies

```shell
npm install
```

4. Run the following commands to compile

```shell
npm run build
```

5. For development, use the following after building

```shell
npm run dev # for web
cargo run   # for native mode
```
