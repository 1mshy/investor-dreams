// src/app/ticker/[symbol]/page.js

"use client"; // Mark this file as a client component

import { usePathname } from 'next/navigation';

const TickerPage = () => {
  const pathname = usePathname();
  const symbol = pathname.split('/').pop(); // Extracts the symbol from the URL
  console.log(pathname)

  return (
    <div>
      <h1>Ticker: {symbol}</h1>
      {/* You can use the symbol variable here to fetch and display data */}
    </div>
  );
};

export default TickerPage;
