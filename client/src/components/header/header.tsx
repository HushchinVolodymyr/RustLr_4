import React from 'react';
import styles from "./header.module.scss";

const Header = () => {
  return (
    <div className={styles.header_container}>
        <h1 className={styles.header_text}>Chat app</h1>
    </div>
  );
};

export default Header;