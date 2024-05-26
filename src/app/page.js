"use client"
import React, { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import "@/app/css/Widgets.css";
import Link from 'next/link';

const Home = () => {
    const router = useRouter();

    useEffect(() => {
        router.push("/home");
    }, [router]);

    return <div>
        <h1>Investor Dreams</h1>
        <h2>HI THERE PERSON</h2>
        <p>You should automatically be moved to the homepage. If that is not the case, click on the button below</p>
        <Link href="/home">Click here!</Link>
    </div>;
};

export default Home;
