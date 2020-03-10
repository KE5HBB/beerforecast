
import { Center, Container, TextInput, Button, Title, Table, Pagination } from "@mantine/core";
import { NextPage } from "next";
import { useEffect, useState, Dispatch, SetStateAction } from "react";
import Service from "../services/transaction";
import { Block } from "../types/block";
import { GambleData } from "../types/gamble";
import styles from "../styles/Dashboard.module.css";
import { useNotifications } from "@mantine/notifications";
const GambleForm = ({
  setTransactionEvent,
  transactionEvent,
}: {
  setTransactionEvent: Dispatch<SetStateAction<number>>;
  transactionEvent: number;
}) => {
  const [value, setVal] = useState("");
  const [balance, setBalance] = useState(0);
  const { showNotification } = useNotifications();
  useEffect(() => {
    getBlanance();
  }, []);
  const getBlanance = async () => {
    let balanceData = await Service.getBlanance();
    setBalance(parseInt(balanceData.balance));
  };
  const gamble = async () => {
    if (!isNaN(parseInt(value))) {
      let res: GambleData = await Service.gamble(parseInt(value));
      if (res.error) {
        showNotification({
          message: "Insufficient Balance or amount too small",
          color: "red",
        });
      } else {
        if (res.win == "true") {
          showNotification({
            message: "You won",
            color: "green",
          });
        } else {
          showNotification({
            message: "You lost",
            color: "red",