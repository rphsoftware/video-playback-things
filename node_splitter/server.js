const chokidar = require('chokidar');
const watcher = chokidar.watch("/home/rph/ramdisk/desktopstreaming/maps");
const fs = require('fs').promises;
const app = require('express')();
const sharp = require('sharp');
const {parse} = require('path');
let b = new Map();

const sleep = timeout => new Promise(resolve => setTimeout(resolve, timeout));

watcher.on('add', async function(path) {
    await sleep(3);
    let data = await fs.readFile(path);
    await fs.unlink(path);

    for (let i = 0; i < 40; i++) {
        b.set(`${i}`, data.slice(i * 16384, (i * 16384) + 16384));
    }
});

app.get("/:u", function(req, res) {
    res.setHeader("Content-Type", "application/octet-stream");
    res.end(b.get(req.params.u));
});

app.listen(5050);