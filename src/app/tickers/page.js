// pages/index.js or pages/playground.js
import dynamic from 'next/dynamic';

const Tinker = dynamic(() => import('../ui_components/pages/TickerPage'), {
    ssr: false
});

export default function GoodTinkerPage() {
    return <Tinker />;
}
