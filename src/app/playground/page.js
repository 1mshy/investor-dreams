// pages/index.js or pages/playground.js
import dynamic from 'next/dynamic';

const Home = dynamic(() => import('../ui_components/containers/Playground'), {
    ssr: false
});

export default function HomePage() {
    return <Home />;
}
