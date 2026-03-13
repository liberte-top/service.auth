import type { LiberteLanguage } from "$lib/i18n/shared";

const copy = {
  en: {
    auth: {
      login: {
        title: "Sign in",
        summary: "We will send a one-time link to your email.",
        submit: "Send sign-in link",
        switchPrompt: "New to Liberte?",
        switchAction: "Create an account",
        verified: "Email verified. You can now request your sign-in link.",
        emailSent: "Check your inbox for your sign-in link.",
      },
      register: {
        title: "Create account",
        summary: "Create your account with email, then verify it to continue.",
        submit: "Create account",
        resend: "Resend verification",
        switchPrompt: "Already have an account?",
        switchAction: "Sign in",
        emailSent: "Check your inbox for the verification email.",
        resendSent: "Verification email sent again.",
      },
      common: {
        emailLabel: "Email address",
        emailPlaceholder: "you@example.com",
        nameLabel: "Name",
        namePlaceholder: "Optional",
        continue: "Continue",
        openProfile: "Open profile",
        logout: "Log out",
        signedIn: "You're already signed in",
        activeSession: "Your account session is active in this browser.",
        supportWithRewrite: "We email a one-time link to continue. After sign-in, you'll return to your original destination.",
        supportDefault: "We email a one-time link to continue. If no destination is provided, you'll land on your profile page.",
        emailRequired: "Email is required.",
        requestFailed: "Request failed with HTTP {status}.",
      },
      profile: {
        title: "Profile",
        description: "View the email address attached to your current Liberte session.",
        subtitle: "This is the email attached to your current session.",
      },
      logout: {
        label: "Logout",
        title: "Log out - Liberte",
        confirmTitle: "Sign out of this session",
        confirmBody: "Use the button below to clear the current browser session.",
        successTitle: "You are signed out",
        successBody: "Your session cookie has been cleared.",
        cancel: "Cancel",
        back: "Back to sign in",
      },
      flow: {
        fallback: "Back to sign in",
        redirecting: "Redirecting in {countdown} seconds.",
        retry: "Return to the sign-in page to request a fresh email.",
        traceLabel: "Trace ID",
        traceHint: "Share this ID when asking for support.",
        verifySuccess: {
          eyebrow: "Email verified",
          title: "Your address is confirmed",
          copy: "You can now continue to sign in with your email link.",
          action: "Continue to sign in",
          titleTag: "Email verified - Liberte",
        },
        verifyInvalid: {
          eyebrow: "Verification issue",
          title: "This verification link is no longer valid",
          copy: "The link may have expired or already been used. Return to the sign-in page and request a fresh email.",
          action: "Request a new verification email",
          titleTag: "Verification link expired - Liberte",
        },
        loginSuccess: {
          eyebrow: "Sign-in complete",
          title: "You are signed in",
          copy: "Your session is active in this browser and you can continue to the requested page.",
          action: "Continue",
          titleTag: "Signed in - Liberte",
        },
        loginInvalid: {
          eyebrow: "Sign-in issue",
          title: "This sign-in link cannot be used anymore",
          copy: "The email link may have expired or already been consumed. Return to the sign-in page to request another one.",
          action: "Request a fresh sign-in link",
          titleTag: "Sign-in link expired - Liberte",
        },
        email: "Email",
        destination: "Destination",
      },
    },
  },
  "zh-CN": {
    auth: {
      login: {
        title: "登录",
        summary: "我们会把一次性登录链接发送到你的邮箱。",
        submit: "发送登录链接",
        switchPrompt: "第一次使用 Liberte？",
        switchAction: "创建账户",
        verified: "邮箱已验证，现在可以请求登录链接了。",
        emailSent: "请前往邮箱查看登录链接。",
      },
      register: {
        title: "创建账户",
        summary: "使用邮箱创建账户，完成验证后继续。",
        submit: "创建账户",
        resend: "重新发送验证邮件",
        switchPrompt: "已经有账户了？",
        switchAction: "去登录",
        emailSent: "请前往邮箱查看验证邮件。",
        resendSent: "验证邮件已重新发送。",
      },
      common: {
        emailLabel: "邮箱地址",
        emailPlaceholder: "you@example.com",
        nameLabel: "名称",
        namePlaceholder: "选填",
        continue: "继续",
        openProfile: "打开个人页",
        logout: "退出登录",
        signedIn: "你已登录",
        activeSession: "当前浏览器中的账户会话仍然有效。",
        supportWithRewrite: "我们会通过邮件发送一次性链接。登录后你将回到原始目标页面。",
        supportDefault: "我们会通过邮件发送一次性链接。如果没有目标地址，登录后将进入个人页。",
        emailRequired: "邮箱不能为空。",
        requestFailed: "请求失败，HTTP 状态码 {status}。",
      },
      profile: {
        title: "个人页",
        description: "查看当前 Liberte 会话绑定的邮箱地址。",
        subtitle: "这是当前会话绑定的邮箱地址。",
      },
      logout: {
        label: "退出",
        title: "退出登录 - Liberte",
        confirmTitle: "退出当前会话",
        confirmBody: "点击下方按钮清除当前浏览器的登录状态。",
        successTitle: "你已退出登录",
        successBody: "当前会话 cookie 已清除。",
        cancel: "取消",
        back: "返回登录",
      },
      flow: {
        fallback: "返回登录",
        redirecting: "将在 {countdown} 秒后跳转。",
        retry: "返回登录页重新请求新的邮件。",
        traceLabel: "Trace ID",
        traceHint: "排查问题时请提供这个编号。",
        verifySuccess: {
          eyebrow: "邮箱已验证",
          title: "你的邮箱已确认",
          copy: "现在可以继续请求登录邮件了。",
          action: "继续登录",
          titleTag: "邮箱已验证 - Liberte",
        },
        verifyInvalid: {
          eyebrow: "验证异常",
          title: "这个验证链接已经失效",
          copy: "链接可能已过期或已被使用。请返回登录页重新请求验证邮件。",
          action: "重新获取验证邮件",
          titleTag: "验证链接已失效 - Liberte",
        },
        loginSuccess: {
          eyebrow: "登录完成",
          title: "你已登录",
          copy: "当前浏览器中的会话已经生效，可以继续前往目标页面。",
          action: "继续",
          titleTag: "已登录 - Liberte",
        },
        loginInvalid: {
          eyebrow: "登录异常",
          title: "这个登录链接已经不能再使用",
          copy: "邮件链接可能已过期或已被消费，请返回登录页重新获取。",
          action: "重新获取登录链接",
          titleTag: "登录链接已失效 - Liberte",
        },
        email: "邮箱",
        destination: "目标地址",
      },
    },
  },
} as const;

export type CopyTree = typeof copy.en;

function getByPath(source: Record<string, unknown>, key: string): string {
  return key.split(".").reduce<unknown>((current, segment) => {
    if (current && typeof current === "object" && segment in current) {
      return (current as Record<string, unknown>)[segment];
    }
    return undefined;
  }, source) as string;
}

export function translate(language: LiberteLanguage, key: string, vars?: Record<string, string | number>) {
  const source = copy[language] ?? copy.en;
  const fallback = getByPath(copy.en as unknown as Record<string, unknown>, key) || key;
  const template = getByPath(source as unknown as Record<string, unknown>, key) || fallback;

  return template.replace(/\{(\w+)\}/g, (_, token: string) => String(vars?.[token] ?? `{${token}}`));
}
