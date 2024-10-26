was going to be just a personal private tool cause one didn't exist at first glance

but then I saw hacktoberfest was on

so I thought I'd make it slightly nicer and put it out there in return for a shirt

Now there is a real tool for getting your card pngs from archidekt since it doesn't let you export those.

### Usage:

```bash
Usage: yank-images [OPTIONS] -i <INPUT>

Options:
  -i <INPUT>               Deck File
  -o <OUTPUT>              Output Directory
  -u, --upscale            Upscale Images if you have `waifu2x-ncnn-vulkan` in path
  -f, --futures <FUTURES>  Number of simultaneous upscaling commands to run
  -h, --help               Print help
  -V, --version            Print version
```

### Example:

1. You go to archidekt.com and export your deck as text, with only set code included
    - (Though it's OK if there's more stuff or if you don't have/care about the set)
2. Run `yank-images -i deck.txt -o ./output-folder`
    - (though you can ommit output folder and it'll create a `./out` for you)
3. Wait for it to download all the images
4. If you have `waifu2x-ncnn-vulkan` in your path, you can upscale the images by adding `--upscale`
    - (You can also specify the number of simultaneous upscaling commands to run with `--futures`, default 10)
    - (It'll output the upscale images to `./output-folder/upscaled`)
    - (Sorry it just upscales 4x with max denoise cause that looks best on my printer and with my eyeballs)
        - (Brother MFC-L3770CDW)
        - (If you need different settings, feel free to write a loop script to run waifu2x yourself or PR command args or something)
