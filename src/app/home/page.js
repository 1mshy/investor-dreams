import dynamic from "next/dynamic";
import React from "react";

const HomeNoSSR = dynamic(() => import("../ui_components/pages/Home"), {
    ssr: false
});

export default function HomePage() {
    return <HomeNoSSR />;
}
