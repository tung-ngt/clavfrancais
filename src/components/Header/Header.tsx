import { Component } from "solid-js";
import styles from "./header.module.css";
import Logo from "../Logo/Logo";
import QuitIcon from "../QuitIcon/QuitIcon";

const Header: Component<{}> = () => {
    return (
        <div class={styles.header}>
            <Logo />
            <button class={styles.quitButton} >
                <QuitIcon width={24} height={24} color="#ff4b55"/>
            </button>
        </div>
    );
};

export default Header;
