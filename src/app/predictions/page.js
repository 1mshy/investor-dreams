import dynamic from 'next/dynamic';

const Predictions = dynamic(() => import('../ui_components/pages/Predictions'), {
    ssr: false
});

export default function PredictionsPage() {
    return <Predictions />;
}
