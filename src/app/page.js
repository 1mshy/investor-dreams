"use client"
import React, { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
/**
 * global css imports
 */
import "@/app/css/Widgets.css"
import { clear_cache, retrieve, store } from './funcs/cache';
import { log } from './funcs/logger';

const Home = () => {
    const router = useRouter();
    // useEffect(() => {
    //     invoke("set_base_size").then((response) => {
    //         console.log("All windows:", response);
    //     });
    // });
    const current_version = "1.0.4"
    useEffect(() => {
        const set_version = retrieve("current_version")
        if (!set_version || set_version !== current_version) {
            log("New version detected, clearing cache", set_version, current_version)
            clear_cache()
            store("current_version", current_version)
        }
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
