import { JSDOM } from 'jsdom';


export async function request_top_companies() {
    console.log("runnins top comps")
    const response = await fetch("https://www.slickcharts.com/sp500");
    const text = await response.text();
    const dom = new JSDOM(text);

    const document = dom.window.document;

    const rows = document.querySelectorAll('table.table tbody tr');
    const data = [];

    rows.forEach((row, index) => {
        const cells = row.querySelectorAll('td');
        if (cells.length >= 4) {
            const number = index + 1;
            const company = cells[1].textContent.trim();
            const ticker_symbol = cells[2].textContent.trim();
            const portfolio_percent = cells[3].textContent.trim();
            data.push({ number, company, ticker_symbol, portfolio_percent });
        }
    });

    console.log(data);
    return data;
}

export async function getServerSideProps() {
    const data = await request_top_companies();
    return {
        props: {
            companies: data,
        },
    };
}