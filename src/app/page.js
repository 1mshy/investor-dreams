"use client"
import React, { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import "@/app/css/Widgets.css";

const Home = () => {
    const router = useRouter();

    useEffect(() => {
        router.push("/main");
    }, [router]);

    return <div></div>;
};

export default Home;
