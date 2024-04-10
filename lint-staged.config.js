export default {
  "*.{js,ts,vue,jsx,tsx,less}": (files) => {
    // 格式化js、ts、vue、jsx、tsx、less文件
    const str = (files || []).join(" ");
    return `prettier --config .prettierrc.cjs --write ${str}`;
  },
  "*.{js,ts,vue,jsx,tsx}": (files) => {
    // 格式化js、ts、vue、jsx、tsx文件
    const str = (files || []).join(" ");
    return `eslint --fix ${str}`;
  },
  "*.{rs}": (files) => {
    // 格式化rust文件
    const str = (files || []).join(" ");
    return `rustfmt ${str}`;
  },
};
