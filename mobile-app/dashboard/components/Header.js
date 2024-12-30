import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

const Header = () => {
    return (
        <View style={styles.container}>
            <Text style={styles.title}>Dashboard Header</Text>
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        padding: 20,
        backgroundColor: '#6200EE',
    },
    title: {
        color: '#FFFFFF',
        fontSize: 20,
        fontWeight: 'bold',
    },
});

export default Header;
