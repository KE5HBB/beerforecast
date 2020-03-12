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
          content="A minimal blockchain protocol written in Rust for begin