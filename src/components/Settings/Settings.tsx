import { Component, createSignal } from "solid-js";
import Panel from "../Panel/Panel";
import ToggleSwitch from "../ToggleSwitch/ToggleSwitch";
import styles from "./settings.module.css";
import Selection from "../Selection/Selection";

const toggleOptions = ["Ctrl + Alt", "Ctrl + Z"];

const Settings: Component<{}> = () => {
    const [runOnStartUp, setRunOnStartUp] = createSignal(false);
    const [hideToTray, setHideToTray] = createSignal(true);
    const [toggleOption, setToggleOption] = createSignal(toggleOptions[0]);

    const toggleRunOnStartUp = () => {
        console.log("changed");
        setRunOnStartUp(!runOnStartUp());
    };

    const toggleHideToTray = () => {
        console.log("changed");
        setHideToTray(!hideToTray());
    };

    const onChangeToggleOption = (option: string) => {
        setToggleOption(option);
    };

    return (
        <section class={styles.section}>
            <h2 class={styles.title}>Settings</h2>

            <Panel maxHeight={300} maxWidth={500}>
                <div class={styles.container}>
                    <ToggleSwitch
                        name="Run on startup"
                        onToggle={toggleRunOnStartUp}
                        checked={runOnStartUp()}
                    />
                    <ToggleSwitch
                        name="Hide to tray"
                        onToggle={toggleHideToTray}
                        checked={hideToTray()}
                    />
                    <Selection
                        name="Toggle language"
                        value={toggleOption()}
                        onchange={onChangeToggleOption}
                        options={toggleOptions}
                    />
                </div>
            </Panel>
        </section>
    );
};

export default Settings;
