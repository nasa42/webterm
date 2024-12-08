export const webtermVersionPlugin = () => {
  return {
    name: "webterm-plugin-version",
    config: () => {
      const version = process.env.npm_package_version || "0.0.0";
      const versionParts = version.split(".");

      return {
        define: {
          "process.env.WEBTERM_VERSION_MAJOR": JSON.stringify(versionParts[0] || "0"),
          "process.env.WEBTERM_VERSION_MINOR": JSON.stringify(versionParts[1] || "0"),
          "process.env.WEBTERM_VERSION_PATCH": JSON.stringify(versionParts[2] || "0"),
        },
      };
    },
  };
};
