# sgbf

![Version: 0.1.0](https://img.shields.io/badge/Version-0.1.0-informational?style=flat-square) ![Type: application](https://img.shields.io/badge/Type-application-informational?style=flat-square) ![AppVersion: 1.0.0](https://img.shields.io/badge/AppVersion-1.0.0-informational?style=flat-square)

A Helm chart for SGBF application (API + Frontend)

## Maintainers

| Name | Email | Url |
| ---- | ------ | --- |
| cfi2017 |  |  |

## Prerequisites

- Kubernetes 1.23+
- Helm 3.0+
- Required secrets (see below)

## Installing the Chart

To install the chart with the release name `sgbf`:

```bash
helm install sgbf ./charts/sgbf
```

To install with custom values:

```bash
helm install sgbf ./charts/sgbf -f custom-values.yaml
```

## Upgrading the Chart

To upgrade an existing release with new image tags:

```bash
helm upgrade sgbf ./charts/sgbf \
  --set api.image.tag=v1.2.3 \
  --set frontend.image.tag=v1.2.3
```

## Uninstalling the Chart

To uninstall/delete the `sgbf` deployment:

```bash
helm uninstall sgbf
```

## Required Secrets

Before deploying, create the following secrets in your namespace:

### Cache Secret
```bash
kubectl create secret generic cache \
  --from-literal=username=<cache-username> \
  --from-literal=password=<cache-password>
```

### OneSignal Secret
```bash
kubectl create secret generic onesignal \
  --from-literal=key=<onesignal-api-key> \
  --from-literal=id=<onesignal-app-id>
```

### Firebase Secret
```bash
kubectl create secret generic firebase \
  --from-file=service-account.json=<path-to-service-account.json>
```

## Values

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| affinity | object | `{}` | Affinity rules for all pods |
| api | object | `{"containerPort":8000,"enabled":true,"env":{"firebaseProject":"sgbf-system","googleApplicationCredentials":"/etc/sgbf/service-account.json","rustLog":"info","sentryDsn":"https://952ffd64359f4f2e83b283faacdd545f@sentry-web.infra-sentry:9000/3"},"image":{"pullPolicy":"IfNotPresent","repository":"ghcr.io/cfi2017/sgbf/api","tag":"latest"},"labels":{"app":"api","swiss.dev/logging":"json"},"name":"api","podSecurityContext":{"fsGroup":65534,"runAsGroup":65534,"runAsNonRoot":true,"runAsUser":65534,"seccompProfile":{"type":"RuntimeDefault"}},"replicaCount":1,"resources":{},"revisionHistoryLimit":3,"secrets":{"cache":{"name":"cache","passwordKey":"password","usernameKey":"username"},"firebase":{"name":"firebase","serviceAccountKey":"service-account.json"},"onesignal":{"idKey":"id","keyKey":"key","name":"onesignal"}},"securityContext":{"allowPrivilegeEscalation":false,"capabilities":{"drop":["ALL"]},"readOnlyRootFilesystem":true,"runAsNonRoot":true,"runAsUser":65534},"service":{"port":80,"targetPort":8000,"type":"ClusterIP"}}` | API service configuration |
| api.containerPort | int | `8000` | API container port |
| api.enabled | bool | `true` | Enable API deployment |
| api.env | object | `{"firebaseProject":"sgbf-system","googleApplicationCredentials":"/etc/sgbf/service-account.json","rustLog":"info","sentryDsn":"https://952ffd64359f4f2e83b283faacdd545f@sentry-web.infra-sentry:9000/3"}` | API environment configuration |
| api.env.firebaseProject | string | `"sgbf-system"` | Firebase project ID |
| api.env.googleApplicationCredentials | string | `"/etc/sgbf/service-account.json"` | Path to Google application credentials file |
| api.env.rustLog | string | `"info"` | Rust log level (trace, debug, info, warn, error) |
| api.env.sentryDsn | string | `"https://952ffd64359f4f2e83b283faacdd545f@sentry-web.infra-sentry:9000/3"` | Sentry DSN for error reporting |
| api.image.pullPolicy | string | `"IfNotPresent"` | API image pull policy |
| api.image.repository | string | `"ghcr.io/cfi2017/sgbf/api"` | API image repository |
| api.image.tag | string | `"latest"` | API image tag (overrides the chart appVersion) |
| api.labels | object | `{"app":"api","swiss.dev/logging":"json"}` | Additional labels for API pods |
| api.name | string | `"api"` | API service name |
| api.podSecurityContext | object | `{"fsGroup":65534,"runAsGroup":65534,"runAsNonRoot":true,"runAsUser":65534,"seccompProfile":{"type":"RuntimeDefault"}}` | Pod security context for API pods (see https://kubernetes.io/docs/tasks/configure-pod-container/security-context/) |
| api.replicaCount | int | `1` | Number of API replicas |
| api.resources | object | `{}` | Resource limits and requests for API pods |
| api.revisionHistoryLimit | int | `3` | Number of revisions to keep |
| api.secrets | object | `{"cache":{"name":"cache","passwordKey":"password","usernameKey":"username"},"firebase":{"name":"firebase","serviceAccountKey":"service-account.json"},"onesignal":{"idKey":"id","keyKey":"key","name":"onesignal"}}` | Secret references for API |
| api.secrets.cache | object | `{"name":"cache","passwordKey":"password","usernameKey":"username"}` | Cache (Redis) credentials secret |
| api.secrets.cache.name | string | `"cache"` | Name of the cache secret |
| api.secrets.cache.passwordKey | string | `"password"` | Key for cache password |
| api.secrets.cache.usernameKey | string | `"username"` | Key for cache username |
| api.secrets.firebase | object | `{"name":"firebase","serviceAccountKey":"service-account.json"}` | Firebase service account secret |
| api.secrets.firebase.name | string | `"firebase"` | Name of the Firebase secret |
| api.secrets.firebase.serviceAccountKey | string | `"service-account.json"` | Key for service account JSON file |
| api.secrets.onesignal | object | `{"idKey":"id","keyKey":"key","name":"onesignal"}` | OneSignal credentials secret |
| api.secrets.onesignal.idKey | string | `"id"` | Key for OneSignal app ID |
| api.secrets.onesignal.keyKey | string | `"key"` | Key for OneSignal API key |
| api.secrets.onesignal.name | string | `"onesignal"` | Name of the OneSignal secret |
| api.securityContext | object | `{"allowPrivilegeEscalation":false,"capabilities":{"drop":["ALL"]},"readOnlyRootFilesystem":true,"runAsNonRoot":true,"runAsUser":65534}` | Container security context for API container (see https://kubernetes.io/docs/tasks/configure-pod-container/security-context/) |
| api.service.port | int | `80` | API service port |
| api.service.targetPort | int | `8000` | API service target port |
| api.service.type | string | `"ClusterIP"` | API service type |
| frontend | object | `{"containerPort":80,"enabled":true,"image":{"pullPolicy":"IfNotPresent","repository":"ghcr.io/cfi2017/sgbf/frontend","tag":"latest"},"labels":{"app":"frontend","swiss.dev/logging":"json"},"name":"frontend","podSecurityContext":{"fsGroup":101,"runAsGroup":101,"runAsNonRoot":true,"runAsUser":101,"seccompProfile":{"type":"RuntimeDefault"}},"replicaCount":1,"resources":{},"revisionHistoryLimit":3,"securityContext":{"allowPrivilegeEscalation":false,"capabilities":{"drop":["ALL"]},"readOnlyRootFilesystem":true,"runAsNonRoot":true,"runAsUser":101},"service":{"port":80,"targetPort":80,"type":"ClusterIP"}}` | Frontend service configuration |
| frontend.containerPort | int | `80` | Frontend container port |
| frontend.enabled | bool | `true` | Enable frontend deployment |
| frontend.image.pullPolicy | string | `"IfNotPresent"` | Frontend image pull policy |
| frontend.image.repository | string | `"ghcr.io/cfi2017/sgbf/frontend"` | Frontend image repository |
| frontend.image.tag | string | `"latest"` | Frontend image tag (overrides the chart appVersion) |
| frontend.labels | object | `{"app":"frontend","swiss.dev/logging":"json"}` | Additional labels for frontend pods |
| frontend.name | string | `"frontend"` | Frontend service name |
| frontend.podSecurityContext | object | `{"fsGroup":101,"runAsGroup":101,"runAsNonRoot":true,"runAsUser":101,"seccompProfile":{"type":"RuntimeDefault"}}` | Pod security context for frontend pods (see https://kubernetes.io/docs/tasks/configure-pod-container/security-context/) |
| frontend.replicaCount | int | `1` | Number of frontend replicas |
| frontend.resources | object | `{}` | Resource limits and requests for frontend pods |
| frontend.revisionHistoryLimit | int | `3` | Number of revisions to keep |
| frontend.securityContext | object | `{"allowPrivilegeEscalation":false,"capabilities":{"drop":["ALL"]},"readOnlyRootFilesystem":true,"runAsNonRoot":true,"runAsUser":101}` | Container security context for frontend container (see https://kubernetes.io/docs/tasks/configure-pod-container/security-context/) |
| frontend.service.port | int | `80` | Frontend service port |
| frontend.service.targetPort | int | `80` | Frontend service target port |
| frontend.service.type | string | `"ClusterIP"` | Frontend service type |
| global | object | `{"imagePullPolicy":"IfNotPresent","linkerd":{"enabled":true}}` | Global settings |
| global.imagePullPolicy | string | `"IfNotPresent"` | Global image pull policy |
| global.linkerd.enabled | bool | `true` | Enable Linkerd service mesh injection for all pods |
| ingress | object | `{"annotations":{},"className":"","enabled":true,"host":"sgbf.swiss.dev","paths":{"api":{"path":"/api","pathType":"Prefix"},"frontend":{"path":"/","pathType":"Prefix"}},"tls":[]}` | Ingress configuration |
| ingress.annotations | object | `{}` | Ingress annotations |
| ingress.className | string | `""` | Ingress class name |
| ingress.enabled | bool | `true` | Enable ingress |
| ingress.host | string | `"sgbf.swiss.dev"` | Ingress hostname |
| ingress.paths | object | `{"api":{"path":"/api","pathType":"Prefix"},"frontend":{"path":"/","pathType":"Prefix"}}` | Ingress path configuration |
| ingress.paths.api | object | `{"path":"/api","pathType":"Prefix"}` | API path configuration |
| ingress.paths.api.path | string | `"/api"` | API path |
| ingress.paths.api.pathType | string | `"Prefix"` | API path type |
| ingress.paths.frontend | object | `{"path":"/","pathType":"Prefix"}` | Frontend path configuration |
| ingress.paths.frontend.path | string | `"/"` | Frontend path |
| ingress.paths.frontend.pathType | string | `"Prefix"` | Frontend path type |
| ingress.tls | list | `[]` | TLS configuration for ingress |
| nodeSelector | object | `{}` | Node selector for all pods |
| podAnnotations | object | `{}` | Pod annotations (applied to all pods) |
| tolerations | list | `[]` | Tolerations for all pods |

## Security Considerations

This chart implements Pod Security Standards with the following defaults:

- **Non-root execution**: Both API and frontend run as non-root users
- **Read-only root filesystem**: Frontend uses read-only root filesystem
- **Seccomp profile**: RuntimeDefault seccomp profile applied
- **Dropped capabilities**: All Linux capabilities are dropped
- **No privilege escalation**: Containers cannot gain additional privileges

You can customize security contexts via `api.podSecurityContext`, `api.securityContext`, `frontend.podSecurityContext`, and `frontend.securityContext` values.

## Examples

### Deploy with specific image tags

```bash
helm upgrade --install sgbf ./charts/sgbf \
  --set api.image.tag=sha-abc123 \
  --set frontend.image.tag=sha-abc123
```

### Enable TLS

```yaml
# custom-values.yaml
ingress:
  tls:
    - secretName: sgbf-tls
      hosts:
        - sgbf.swiss.dev
```

```bash
helm upgrade --install sgbf ./charts/sgbf -f custom-values.yaml
```

### Disable Linkerd injection

```bash
helm upgrade --install sgbf ./charts/sgbf \
  --set global.linkerd.enabled=false
```

### Set resource limits

```yaml
# custom-values.yaml
api:
  resources:
    limits:
      cpu: 500m
      memory: 512Mi
    requests:
      cpu: 100m
      memory: 128Mi

frontend:
  resources:
    limits:
      cpu: 200m
      memory: 256Mi
    requests:
      cpu: 50m
      memory: 64Mi
```

```bash
helm upgrade --install sgbf ./charts/sgbf -f custom-values.yaml
```

## GitOps Integration

For production deployments, consider using GitOps tools:

### ArgoCD Application

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: sgbf
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/cfi2017/sgbf
    targetRevision: main
    path: charts/sgbf
    helm:
      values: |
        api:
          image:
            tag: v1.2.3
        frontend:
          image:
            tag: v1.2.3
  destination:
    server: https://kubernetes.default.svc
    namespace: default
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
```

### Flux HelmRelease

```yaml
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: sgbf
  namespace: default
spec:
  interval: 5m
  chart:
    spec:
      chart: ./charts/sgbf
      sourceRef:
        kind: GitRepository
        name: sgbf
        namespace: flux-system
  values:
    api:
      image:
        tag: v1.2.3
    frontend:
      image:
        tag: v1.2.3
```

----------------------------------------------
Autogenerated from chart metadata using [helm-docs v1.14.2](https://github.com/norwoodj/helm-docs/releases/v1.14.2)
