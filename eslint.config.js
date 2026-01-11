import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import security from 'eslint-plugin-security';

export default tseslint.config(
    js.configs.recommended,
    ...tseslint.configs.recommendedTypeChecked,
    {
        languageOptions: {
            parserOptions: {
                project: true,
                tsconfigRootDir: import.meta.dirname,
            },
        },
    },
    {
        plugins: {
            react,
            'react-hooks': reactHooks,
            security,
        },
        settings: {
            react: {
                version: 'detect',
            },
        },
        rules: {
            // Security rules - ALWAYS ERROR
            'no-eval': 'error',
            'no-implied-eval': 'error',
            'no-new-func': 'error',
            'security/detect-object-injection': 'warn',
            'security/detect-non-literal-regexp': 'error',
            'security/detect-unsafe-regex': 'error',

            // TypeScript strict rules
            '@typescript-eslint/no-explicit-any': 'error',
            '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
            '@typescript-eslint/explicit-function-return-type': 'warn',
            '@typescript-eslint/no-floating-promises': 'error',
            '@typescript-eslint/no-misused-promises': 'error',
            '@typescript-eslint/await-thenable': 'error',

            // React rules
            'react/react-in-jsx-scope': 'off',
            'react/prop-types': 'off',
            'react-hooks/rules-of-hooks': 'error',
            'react-hooks/exhaustive-deps': 'warn',
        },
    },
    {
        ignores: ['node_modules/', 'dist/', 'src-tauri/', '*.config.js', '*.config.ts'],
    }
);
