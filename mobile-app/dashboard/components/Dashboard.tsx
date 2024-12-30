import React from 'react';
import { View, StyleSheet } from 'react-native';
import Header from './Header';
import TransactionList from './TransactionList';

const Dashboard: React.FC = () => {
    const transactions = [
        { id: 1, description: 'Groceries', amount: 50 },
        { id: 2, description: 'Gas', amount: 30 },
        { id: 3, description: 'Rent', amount: 1200 },
    ];

    return (
        <View style={styles.container}>
            <Header />
            <TransactionList transactions={transactions} />
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#f5f5f5',
    },
});

export default Dashboard;
