# rss-tube
### Rust CLI script to use youtube from RSS, download videos for later

###### Currently only tested on linux

## Features:
- Fully offline
- subscriptions stored in a OPML file
- Import subscriptions from [youtube export](https://newpipe.net/FAQ/tutorials/import-export-data/#import-youtube)
- Specify max publish date and video length

---

## How to start
1. Download [yt-dlp](https://github.com/yt-dlp/yt-dlp). Make sure its available system wide
2. get the subscriptions.csv from [youtube export](https://newpipe.net/FAQ/tutorials/import-export-data/#import-youtube)
3. Run the binary with arguments: ./rss-tube ----csv-to-opml csv_file new_opml_file
4. Done, your opml file is ready

---

## Available arguments:
```
--file-name [Path] - Specifies the path and file name to the OPML file
--links-directory [Path] - Specifies the path and folder name to where save XML files (rss information of specified channels)
--download-directory [Path] - Specifies the path and folder name to where save videos
--set-categories \"category1,category2,category3\" - Chooses from whot categories from the OPML to file download. the --csv-to-opml does not create any categores. this can be done in Liferea for example
--yt-dlp-arguments \"argument1,argument2,argument3\" - Specifies custom arguments for yt-dlp
--time \"YYYY,MM,DD,HH\" - Specifies time that older than it, videos will be ignored and not downloaded. HH means Hours of the day
--channel-link [url] - Turns a yt channel link to a rss link to that channel. if xclip is installed, it puts it to clipboard
--help - shows this message
--max-video-time - Specifies the maximum time of a video, in minutes
--csv-to-opml [csv_file] [new_opml_file] - Converts the CSV file from youtube export subscription.csv file to a opml file, ready to use with this program. The syntax is: --csv-to-opml csv_file new_opml_file"
```

---

## Things to be added in the future:
- Add windows support
- write categories as folders in download folders
- show available categories from file
- more default yt-dlp arguments like --quality --memory_low
- less .clone() etc.
- interactive mode ( download this video yes/no)
- last usage save to config file ( to restore on exact hour )
- better formatting ( calculating chars and line capacity)

---

### Contrubiting, Feature requests, Suggestions are welcome 
