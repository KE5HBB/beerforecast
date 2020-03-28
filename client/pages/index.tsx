import type { NextPage } from "next";
import Link from "next/link";
import styles from "../styles/Home.module.css";
import { Button } from "@mantine/core";
import Particles from "react-tsparticles";
import Head from "next/head";
const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>Night Protocol</title>
        <meta
          name="description"
          content="A minimal blockchain protocol written in Rust for beginners to understand the modern infrastructure of blockchain."
        />
        <meta property="og:title" content="Night Protocol" />
        <meta
          property="og:description"
          content="A minimal blockchain protocol written in Rust for beginners to understand the modern infrastructure of blockchain."
        />
      </Head>
      <div className={styles.top_container}>
        <div>
          <h1 className={styles.heading}>Night Protocol</h1>
          <p className={styles.description}>
            A minimal blockchain protocol written in Rust for beginners to
            understand <br /> the modern infrastructure of blockchain.
          </p>
          <Link href="/explore">
            <Button
              size="lg"
              className={styles.text}
              variant="gradient"
              gradient={{ from: "o