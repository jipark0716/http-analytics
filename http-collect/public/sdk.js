"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
const DEVICE_ID_KEY = 'analytics_device_id';
const SESSION_KEY = 'analytics_session';
const IS_LOGIN = 'analytics_is_login';
class AnalyticsClient {
    constructor(client_id, endpoint) {
        this.client_id = client_id;
        this.endpoint = endpoint;
        this.login_id = null;
        const deviceId = localStorage.getItem(DEVICE_ID_KEY);
        if (!deviceId) {
            this.device_id = crypto.randomUUID();
            localStorage.setItem(DEVICE_ID_KEY, this.device_id);
        }
        else {
            this.device_id = deviceId;
        }
    }
    load() {
        return __awaiter(this, void 0, void 0, function* () {
            setInterval(() => __awaiter(this, void 0, void 0, function* () {
                var _a;
                if (!localStorage.getItem(IS_LOGIN)) {
                    // 로그인 이벤트 송신 안했으면
                    const customer = (_a = window.CAPP_ASYNC_METHODS) === null || _a === void 0 ? void 0 : _a.AppCommon.getCustomerInfo();
                    if (!customer)
                        return;
                    localStorage.setItem(IS_LOGIN, "1");
                    this.login_id = customer.member_id;
                    yield this.create_event("/api/v1/events/auth/login", {
                        login_id: this.login_id,
                        phone_number: customer.cellphone,
                    });
                }
            }), 500);
        });
    }
    get_expired_at() {
        const now = new Date();
        return new Date(now.getTime() + 30 * 60 * 1000);
    }
    create_session() {
        return __awaiter(this, void 0, void 0, function* () {
            const response = yield fetch(`${this.endpoint}/api/v1/sessions`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    client_id: this.client_id,
                    device_id: this.device_id,
                }),
            });
            const createSessionResponse = yield response.json();
            const session = {
                uuid: createSessionResponse.uuid,
                expired_at: this.get_expired_at(),
            };
            localStorage.setItem(SESSION_KEY, JSON.stringify(session));
            return session;
        });
    }
    pre_event() {
        return __awaiter(this, void 0, void 0, function* () {
            // 세션 존재 확인
            const sessionStr = localStorage.getItem(SESSION_KEY);
            if (!sessionStr) {
                return (yield this.create_session()).uuid;
            }
            const session = JSON.parse(sessionStr);
            // 세션 만료 확인
            if (session.expired_at < new Date()) {
                return (yield this.create_session()).uuid;
            }
            return session.uuid;
        });
    }
    send(path, body) {
        navigator.sendBeacon(`${this.endpoint}${path}`, new Blob([
            JSON.stringify(body)
        ], { type: 'application/json' }));
    }
    create_event(path, payload) {
        return __awaiter(this, void 0, void 0, function* () {
            this.send(path, Object.assign(Object.assign({}, payload), {
                uuid: yield this.pre_event(),
                client_id: this.client_id,
            }));
        });
    }
    create_event_forget(path, payload) {
        // 세션 존재 확인
        const sessionStr = localStorage.getItem(SESSION_KEY);
        if (!sessionStr) {
            return;
        }
        const session = JSON.parse(sessionStr);
        this.send(path, Object.assign(Object.assign({}, payload), {
            uuid: session.uuid,
            client_id: this.client_id,
        }));
    }
}
window.HTTP_ANALYTICS = new AnalyticsClient(1, "http://127.0.0.1:8080");
const onLoad = (callback) => {
    if (document.readyState === 'complete') {
        callback();
    }
    else {
        window.addEventListener('load', callback);
    }
};
onLoad(() => { var _a; return (_a = window.HTTP_ANALYTICS) === null || _a === void 0 ? void 0 : _a.load(); });
class MainCollector {
    init(path) {
        var _a;
        (_a = window.HTTP_ANALYTICS) === null || _a === void 0 ? void 0 : _a.create_event('/api/v1/events/main/view-start', {});
    }
    scroll(bucket) {
        var _a;
        (_a = window.HTTP_ANALYTICS) === null || _a === void 0 ? void 0 : _a.create_event('/api/v1/events/main/scroll', {
            scroll_bucket: bucket,
        });
    }
    close(path) {
        var _a;
        (_a = window.HTTP_ANALYTICS) === null || _a === void 0 ? void 0 : _a.create_event_forget('/api/v1/events/main/view-end', {});
    }
}
class ProductDetailCollector {
    constructor(search) {
        const productNo = search.get('product_no');
        if (!productNo) {
            throw new Error('No product not found');
        }
        this.product_id = productNo;
    }
    init(path) {
        var _a;
        (_a = window.HTTP_ANALYTICS) === null || _a === void 0 ? void 0 : _a.create_event('/api/v1/events/product/view-start', {
            product_id: this.product_id,
        });
    }
    scroll(bucket) {
        var _a;
        (_a = window.HTTP_ANALYTICS) === null || _a === void 0 ? void 0 : _a.create_event('/api/v1/events/product/scroll', {
            scroll_bucket: bucket,
            product_id: this.product_id,
        });
    }
    close(path) {
        var _a;
        (_a = window.HTTP_ANALYTICS) === null || _a === void 0 ? void 0 : _a.create_event_forget('/api/v1/events/product/view-end', {
            product_id: this.product_id,
        });
    }
}
let collector = null;
if (['/', '/index.html'].includes(location.pathname)) {
    collector = new MainCollector();
}
else if (['/product/detail.html'].includes(location.pathname)) {
    collector = new ProductDetailCollector(new URLSearchParams(location.search));
}
else {
}
collector === null || collector === void 0 ? void 0 : collector.init(location.pathname);
window.addEventListener('pagehide', () => {
    collector === null || collector === void 0 ? void 0 : collector.close(location.pathname);
});
let last_scroll_bucket = 0;
let next_scroll_y = 500;
window.addEventListener('scroll', () => {
    const scrollY = window.scrollY || document.documentElement.scrollTop;
    if (scrollY >= next_scroll_y) {
        next_scroll_y = Math.ceil(next_scroll_y * 1.6);
        collector === null || collector === void 0 ? void 0 : collector.scroll(++last_scroll_bucket);
    }
});
