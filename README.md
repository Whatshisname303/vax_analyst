# Vax Analyst

The default stats page in Kovaaks is ridiculously slow for any scenarios if you have played it more than a few times. I decided to make a tool that's much faster since it's written in rust and doesn't do anything but let you look at your data.

If you've got a lot of time in Kovaaks then the file reads will still take a while to initially load since each scenario run is stored as a CSV, but this data gets cached so running the program for the first time might take a minute or two, but after that it should be practically instant. I've thought of some ways to rewrite the file loading by either doing it incrementally while the program is running, or reading different sections in parallel. I'm not sure how much of a speedup that would get, and I just haven't bothered to check it out too much since it isn't that annoying for myself at least right now.

I haven't implemented all the same features. There aren't any trendlines or data grouping by time periods, but there are a few features that I've added that are missing in Kovaaks like sorting scenarios by plays and counting general data like unique scenarios. I'm in college right now and not grinding too much Kovaaks, but might add some more features once I'm home and put in some more time.

# Usage

Run the project

`cargo run`

You'll initially get a screen to give the path to your Kovaaks stats which should be under something like `steamapps/common/FPSAimTrainer/FPSAimTrainer/stats` (the same spot you would go to add custom crosshairs, sounds, or any of that).

<img width="616" alt="image" src="https://github.com/user-attachments/assets/0bf9021c-deee-4388-8fab-4f516c9dedbe">

I copied over my stats into the project for testing which is why I'm using `./stats` but you should probably use the full path. Once you've got the `Path successfully updated` message, just restart the program and your data should start loading.

You can update the path and a few other things in the config. The watch run screen might be implemented later to let you see performance stats on each run as you play them, but it has nothing right now. On the right you can search scenarios and sort by plays to get some general data, then you can generate a graph for each that creates a scatterplot of all your plays from your first to your most recent.
