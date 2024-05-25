"use client"
import React, { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import "@/app/css/Widgets.css";
import Link from 'next/link';

const Home = () => {
    const router = useRouter();

    // useEffect(() => {
    //     router.push("/home");
    // }, [router]);

    return <div>
        <h1>Investor Dreams</h1>
        <h2>HI THERE PERSON</h2>
        <Link href="/home"> Click here</Link>
    </div>;
};

export default Home;
