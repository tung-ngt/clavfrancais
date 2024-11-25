import { Component, createSignal } from "solid-js";
import styles from "./language-picker.module.css";
import FlagButton from "../FlagButton/FlagButton";
import Panel from "../Panel/Panel";

enum Language {
    English,
    French,
}
const LanguagePicker: Component<{}> = () => {
    const [language, setLanguage] = createSignal<Language>(Language.English);

    const handleLanguageChange = (language: Language) => {
        setLanguage(language);
    };
    return (
        <section class={styles.section}>

        <Panel maxWidth={500} maxHeight={300}>
            <div class={styles.flagContainer}>
                <FlagButton
                    name="French"
                    imagePath="/france.png"
                    isActive={language() == Language.French}
                    onClick={() => handleLanguageChange(Language.French)}
                ></FlagButton>
                <FlagButton
                    name="English"
                    imagePath="/uk.png"
                    isActive={language() == Language.English}
                    onClick={() => setLanguage(Language.English)}
                ></FlagButton>
            </div>
        </Panel>
        </section>
    );
};

export default LanguagePicker;
