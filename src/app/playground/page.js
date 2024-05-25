// pages/index.js or pages/playground.js
import dynamic from 'next/dynamic';

const Home = dynamic(() => import('../ui_components/pages/Playground'), {
    ssr: false
});

export default function PlaygroundPage() {
    return <Home />;
}
