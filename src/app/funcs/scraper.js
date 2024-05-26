
import { invoke } from '@tauri-apps/api';
import cheerio from 'cheerio';

export async function request_top_companies() {
    console.log("runnins top comps")
    const text = await invoke('get_index_info');
    const $ = cheerio.load(text);
    const data = [];
    $('table.table tbody tr').each((index, element) => {
        const cells = $(element).find('td');
        if (cells.length >= 4) {
            const number = index + 1;
            const company = $(cells[1]).text().trim();
            const ticker_symbol = $(cells[2]).text().trim();
            const portfolio_percent = $(cells[3]).text().trim();
            data.push({ number, company, ticker_symbol, portfolio_percent });
        }
    });

    console.log(data);
    return data;
}
