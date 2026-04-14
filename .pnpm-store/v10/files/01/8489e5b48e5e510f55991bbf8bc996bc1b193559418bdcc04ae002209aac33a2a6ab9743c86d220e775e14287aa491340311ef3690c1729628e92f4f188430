import { App } from 'vue';

export declare const ColorPicker: import("vue").DefineComponent<{
    color: {
        type: StringConstructor;
        default: string;
    };
    theme: {
        type: StringConstructor;
        default: string;
    };
    suckerHide: {
        type: BooleanConstructor;
        default: boolean;
    };
    suckerCanvas: {
        type: null;
        default: null;
    };
    suckerArea: {
        type: ArrayConstructor;
        default: () => never[];
    };
    colorsDefault: {
        type: ArrayConstructor;
        default: () => string[];
    };
    colorsHistoryKey: {
        type: StringConstructor;
        default: string;
    };
}, unknown, {
    hueWidth: number;
    hueHeight: number;
    previewHeight: number;
    modelRgba: string;
    modelHex: string;
    r: number;
    g: number;
    b: number;
    a: number;
    h: number;
    s: number;
    v: number;
}, {
    isLightTheme(): boolean;
    totalWidth(): number;
    previewWidth(): number;
    rgba(): object;
    hsv(): object;
    rgbString(): string;
    rgbaStringShort(): string;
    rgbaString(): string;
    hexString(): string;
}, {
    selectSaturation(color: any): void;
    handleFocus(event: FocusEvent): void;
    handleBlur(event: FocusEvent): void;
    selectHue(color: any): void;
    selectAlpha(a: any): void;
    inputHex(color: string): void;
    inputRgba(color: string): void;
    setText(): void;
    openSucker(isOpen: boolean): void;
    selectSucker(color: string): void;
    selectColor(color: string): void;
}, import("vue").ComponentOptionsMixin, import("vue").ComponentOptionsMixin, ("changeColor" | "openSucker" | "inputFocus" | "inputBlur")[], "changeColor" | "openSucker" | "inputFocus" | "inputBlur", import("vue").VNodeProps & import("vue").AllowedComponentProps & import("vue").ComponentCustomProps, Readonly<{
    color: string;
    theme: string;
    suckerHide: boolean;
    suckerCanvas: any;
    suckerArea: unknown[];
    colorsDefault: unknown[];
    colorsHistoryKey: string;
} & {}>, {
    color: string;
    theme: string;
    suckerHide: boolean;
    suckerCanvas: any;
    suckerArea: unknown[];
    colorsDefault: unknown[];
    colorsHistoryKey: string;
}>;

declare const _default: {
    install: typeof install;
};
export default _default;

declare function install(Vue: App): void;

export { }
