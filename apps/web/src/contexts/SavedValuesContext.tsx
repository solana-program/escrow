'use client';

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';

const STORAGE_KEY = 'escrow-ui-saved-values-v1';
const MAX_SAVED_VALUES = 25;

interface SavedValuesState {
    defaultEscrow: string;
    defaultMint: string;
    defaultReceipt: string;
    escrows: string[];
    mints: string[];
    receipts: string[];
}

const INITIAL_STATE: SavedValuesState = {
    defaultEscrow: '',
    defaultMint: '',
    defaultReceipt: '',
    escrows: [],
    mints: [],
    receipts: [],
};

interface SavedValuesContextType extends SavedValuesState {
    setDefaultEscrow: (value: string) => void;
    setDefaultMint: (value: string) => void;
    setDefaultReceipt: (value: string) => void;
    rememberEscrow: (value: string) => void;
    rememberMint: (value: string) => void;
    rememberReceipt: (value: string) => void;
    clearSavedValues: () => void;
}

const SavedValuesContext = createContext<SavedValuesContextType | null>(null);

function normalizeValue(value: string) {
    return value.trim();
}

function addUnique(values: string[], value: string): string[] {
    const normalized = normalizeValue(value);
    if (!normalized) return values;
    return [normalized, ...values.filter(v => v !== normalized)].slice(0, MAX_SAVED_VALUES);
}

function isSavedValuesState(value: unknown): value is SavedValuesState {
    if (!value || typeof value !== 'object') return false;
    const candidate = value as Record<string, unknown>;
    return (
        typeof candidate.defaultEscrow === 'string' &&
        typeof candidate.defaultMint === 'string' &&
        typeof candidate.defaultReceipt === 'string' &&
        Array.isArray(candidate.escrows) &&
        Array.isArray(candidate.mints) &&
        Array.isArray(candidate.receipts)
    );
}

function readFromStorage(): SavedValuesState {
    try {
        const raw = window.localStorage.getItem(STORAGE_KEY);
        if (!raw) return INITIAL_STATE;
        const parsed: unknown = JSON.parse(raw);
        if (!isSavedValuesState(parsed)) return INITIAL_STATE;
        return {
            defaultEscrow: normalizeValue(parsed.defaultEscrow),
            defaultMint: normalizeValue(parsed.defaultMint),
            defaultReceipt: normalizeValue(parsed.defaultReceipt),
            escrows: parsed.escrows
                .map(v => normalizeValue(String(v)))
                .filter(Boolean)
                .slice(0, MAX_SAVED_VALUES),
            mints: parsed.mints
                .map(v => normalizeValue(String(v)))
                .filter(Boolean)
                .slice(0, MAX_SAVED_VALUES),
            receipts: parsed.receipts
                .map(v => normalizeValue(String(v)))
                .filter(Boolean)
                .slice(0, MAX_SAVED_VALUES),
        };
    } catch {
        return INITIAL_STATE;
    }
}

export function SavedValuesProvider({ children }: { children: React.ReactNode }) {
    const [state, setState] = useState<SavedValuesState>(INITIAL_STATE);
    const [hydrated, setHydrated] = useState(false);

    useEffect(() => {
        setState(readFromStorage());
        setHydrated(true);
    }, []);

    useEffect(() => {
        if (!hydrated) return;
        window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    }, [state, hydrated]);

    const setDefaultEscrow = useCallback((value: string) => {
        setState(current => ({ ...current, defaultEscrow: normalizeValue(value) }));
    }, []);

    const setDefaultMint = useCallback((value: string) => {
        setState(current => ({ ...current, defaultMint: normalizeValue(value) }));
    }, []);

    const setDefaultReceipt = useCallback((value: string) => {
        setState(current => ({ ...current, defaultReceipt: normalizeValue(value) }));
    }, []);

    const rememberEscrow = useCallback((value: string) => {
        setState(current => {
            const normalized = normalizeValue(value);
            if (!normalized) return current;
            return {
                ...current,
                defaultEscrow: normalized,
                escrows: addUnique(current.escrows, normalized),
            };
        });
    }, []);

    const rememberMint = useCallback((value: string) => {
        setState(current => {
            const normalized = normalizeValue(value);
            if (!normalized) return current;
            return {
                ...current,
                defaultMint: normalized,
                mints: addUnique(current.mints, normalized),
            };
        });
    }, []);

    const rememberReceipt = useCallback((value: string) => {
        setState(current => {
            const normalized = normalizeValue(value);
            if (!normalized) return current;
            return {
                ...current,
                defaultReceipt: normalized,
                receipts: addUnique(current.receipts, normalized),
            };
        });
    }, []);

    const clearSavedValues = useCallback(() => {
        setState(INITIAL_STATE);
    }, []);

    const contextValue = useMemo<SavedValuesContextType>(
        () => ({
            ...state,
            setDefaultEscrow,
            setDefaultMint,
            setDefaultReceipt,
            rememberEscrow,
            rememberMint,
            rememberReceipt,
            clearSavedValues,
        }),
        [
            state,
            setDefaultEscrow,
            setDefaultMint,
            setDefaultReceipt,
            rememberEscrow,
            rememberMint,
            rememberReceipt,
            clearSavedValues,
        ],
    );

    return <SavedValuesContext.Provider value={contextValue}>{children}</SavedValuesContext.Provider>;
}

export function useSavedValues() {
    const context = useContext(SavedValuesContext);
    if (!context) {
        throw new Error('useSavedValues must be used inside SavedValuesProvider');
    }
    return context;
}
