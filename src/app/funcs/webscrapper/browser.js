const puppeteer = require('puppeteer');

async function startBrowser() {
    let browser;
    try {
        console.log("Opened")
        browser = await puppeteer.launch({
            headless: false,
            args: ["--disable-setuid-sandbox"],
            'ignoreHTTPSErrors': true
        });
    } catch (ex) {
        console.log("Could not create browser instance: ", ex)
    }
    return browser;
}

module.exports = {
    startBrowser
};