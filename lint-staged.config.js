export default {
  "*.{js,ts,vue,jsx,tsx,less}": ["yarn prettier"],
  "*.{js,ts,vue,jsx,tsx}": ["yarn lint:es"],
  "*.{rs}": (files) => {
    // 格式化rust文件
    const str = (files || []).join(" ");
    return `rustfmt ${str}`;
  },
};
