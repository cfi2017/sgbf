# SGBF Helm Chart

This Helm chart deploys the SGBF application stack, which consists of an API service and a frontend service.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+
- Secrets must be created before deploying:
  - `cache` - Redis/cache credentials
  - `onesignal` - OneSignal API credentials
  - `firebase` - Firebase service account JSON

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

## Configuration

The following table lists the configurable parameters of the SGBF chart and their default values.

### Global Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `global.imagePullPolicy` | Global image pull policy | `IfNotPresent` |
| `global.linkerd.enabled` | Enable Linkerd service mesh injection | `true` |

### API Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `api.enabled` | Enable API deployment | `true` |
| `api.replicaCount` | Number of API replicas | `1` |
| `api.image.repository` | API image repository | `ghcr.io/cfi2017/sgbf/api` |
| `api.image.tag` | API image tag | `latest` |
| `api.containerPort` | API container port | `8000` |
| `api.service.type` | API service type | `ClusterIP` |
| `api.service.port` | API service port | `80` |
| `api.env.sentryDsn` | Sentry DSN for error reporting | `""` |
| `api.env.rustLog` | Rust log level | `info` |
| `api.env.firebaseProject` | Firebase project ID | `sgbf-system` |

### Frontend Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `frontend.enabled` | Enable frontend deployment | `true` |
| `frontend.replicaCount` | Number of frontend replicas | `1` |
| `frontend.image.repository` | Frontend image repository | `ghcr.io/cfi2017/sgbf/frontend` |
| `frontend.image.tag` | Frontend image tag | `latest` |
| `frontend.containerPort` | Frontend container port | `80` |
| `frontend.service.type` | Frontend service type | `ClusterIP` |
| `frontend.service.port` | Frontend service port | `80` |

### Ingress Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `ingress.enabled` | Enable ingress | `true` |
| `ingress.className` | Ingress class name | `""` |
| `ingress.host` | Ingress hostname | `sgbf.swiss.dev` |
| `ingress.paths.api.path` | API path | `/api` |
| `ingress.paths.frontend.path` | Frontend path | `/` |
| `ingress.tls` | TLS configuration | `[]` |

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
